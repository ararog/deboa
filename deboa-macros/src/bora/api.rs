extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::{format_ident, quote};
use regex::Regex;
use syn::{Data, DeriveInput, Ident, LitStr, Type, TypeTuple, Visibility, parse_macro_input, parse_str, punctuated::Punctuated, token::Paren};

use bora::parser::BoraApi;
use titlecase::Titlecase;

use crate::{
    bora::{self, parser::OperationEnum},
    parser::operations::{delete::DeleteFieldEnum, get::GetFieldEnum, patch::PatchFieldEnum, post::PostFieldEnum, put::PutFieldEnum},
};

fn extract_path_params(path: &str) -> String {
    Regex::new(r"<(\w*):\&{0,1}\w*>")
        .expect("Invalid path")
        .replace_all(path, "{$1}")
        .to_string()
}

fn extract_params_from_path(path: &LitStr) -> (TS2, LitStr) {
    let raw_path = path.value();
    let params = Regex::new(r"<(\w*:\&{0,1}\w*)>")
        .unwrap()
        .captures_iter(&raw_path)
        .map(|m| m.get(1).unwrap().as_str())
        .collect::<Vec<_>>();

    let api_params = params.clone().into_iter().fold(TS2::new(), |mut acc, param| {
        let pair = param.split(':').collect::<Vec<_>>();
        let param = parse_str::<syn::Ident>(pair[0]).unwrap();
        let param_type = parse_str::<syn::Type>(pair[1]).unwrap();
        acc.extend(quote! {
            #param: #param_type,
        });
        acc
    });

    let has_query = raw_path.contains("?");
    let new_path = if has_query {
        let parts = raw_path.split("?").collect::<Vec<_>>();
        let path = parts[0];
        let query = parts[1];

        let path_output = extract_path_params(path);

        let query_regex = Regex::new(r"<(\w*:\&{0,1}\w*)>").expect("Invalid query");
        let query_output = query_regex
            .captures_iter(query)
            .map(|m| {
                let pair = m.get(1).unwrap().as_str().split(':').collect::<Vec<_>>();
                format!("{}={{{}}}", pair[0], pair[0])
            })
            .collect::<Vec<_>>()
            .join("&");
        format!("{path_output}?{query_output}")
    } else {
        extract_path_params(&raw_path)
    };

    (api_params, LitStr::new(&new_path, Span::call_site()))
}

#[allow(clippy::too_many_arguments)]
fn impl_function(
    deboa_method: &Ident,
    format_module: &Ident,
    api_path: &LitStr,
    method_name: &Ident,
    api_params: &TS2,
    req_body_type: &Type,
    res_body_type: &Type,
    unit_type: &Type,
) -> TS2 {
    if res_body_type.eq(unit_type) {
        quote! {
            pub async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<#res_body_type, DeboaError> {
                self.api.#deboa_method(format!(#api_path).as_ref())?.go(self.api.client()).await?;
                Ok(())
            }
        }
    } else {
        quote! {
            pub async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<#res_body_type, DeboaError> {
                self.api.#deboa_method(format!(#api_path).as_ref())?.go(self.api.client()).set_body_as(#format_module, body)?.await?.body_as(#format_module)
            }
        }
    }
}

pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let attr = TS2::from(attr);

    let parse_attr = attr.clone().into();
    let root = parse_macro_input!(parse_attr as BoraApi);

    let name = if let Visibility::Public(_) = item.vis
        && let Data::Struct(_) = item.data
    {
        item.ident.to_string()
    } else {
        panic!("expected public struct");
    };

    let struct_name = parse_str::<syn::Ident>(name.as_str()).unwrap();

    let mut imports = TS2::new();
    let mut struct_impl = TS2::new();
    let unit_type = Type::Tuple(TypeTuple {
        paren_token: Paren::default(),
        elems: Punctuated::new(),
    });

    root.operations.iter().fold((&mut imports, &mut struct_impl), |acc, op| {
        match op {
            OperationEnum::get(get) => {
                let fields = &get.fields;

                let method = parse_str::<syn::Ident>("get").unwrap();
                let mut method_name = Ident::new("ident", Span::call_site());
                let mut api_path = LitStr::new("lit", Span::call_site());
                let mut res_body_type = Type::Verbatim(TS2::new());
                let mut api_params = TS2::new();
                let mut format_name = Ident::new("ident", Span::call_site());
                let mut format_module = Ident::new("ident", Span::call_site());

                fields.iter().for_each(|field| match field {
                    GetFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), Span::call_site());
                    }
                    GetFieldEnum::path(path) => {
                        let path_info = extract_params_from_path(&path.value);

                        api_path = path_info.1;
                        api_params = path_info.0;
                    }
                    GetFieldEnum::res_body(res_body) => {
                        res_body_type = res_body.value.clone();
                    }
                    GetFieldEnum::format(format) => {
                        let format_value = format.value.value();
                        format_name = format_ident!("{}", format_value);
                        format_module = format_ident!("{}Body", format_value.titlecase());
                    }
                });

                if acc.0.is_empty() {
                    acc.0.extend(quote! {
                        use deboa_extras::http::serde::#format_name::{#format_module};
                    });
                }

                acc.1.extend(quote! {
                    pub async fn #method_name(&mut self, #api_params) -> Result<#res_body_type, DeboaError> {
                        self.api.#method(format!(#api_path).as_ref())?.go(&mut self.api.client()).await?.body_as(#format_module)
                    }
                });
            }
            OperationEnum::post(post) => {
                let fields = &post.fields;

                let method = parse_str::<syn::Ident>("post").unwrap();
                let mut method_name = Ident::new("ident", Span::call_site());
                let mut api_path = LitStr::new("lit", Span::call_site());
                let mut req_body_type = Type::Verbatim(TS2::new());
                let mut res_body_type = &unit_type;
                let mut api_params = TS2::new();
                let mut format_name = Ident::new("ident", Span::call_site());
                let mut format_module = Ident::new("ident", Span::call_site());

                fields.iter().for_each(|field| match field {
                    PostFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), Span::call_site());
                    }
                    PostFieldEnum::path(path) => {
                        let path_info = extract_params_from_path(&path.value);

                        api_path = path_info.1;
                        api_params = path_info.0;
                    }
                    PostFieldEnum::req_body(req_body) => {
                        req_body_type = req_body.value.clone();
                    }
                    PostFieldEnum::res_body(res_body) => {
                        res_body_type = &res_body.value;
                    }
                    PostFieldEnum::format(format) => {
                        let format_value = format.value.value();
                        let title_format_value = format_value.titlecase();
                        format_name = format_ident!("{}", format_value);
                        format_module = format_ident!("{}Body", title_format_value);
                    }
                });

                if acc.0.is_empty() {
                    acc.0.extend(quote! {
                        use deboa_extras::http::serde::#format_name::{#format_module};
                    });
                }

                if res_body_type.eq(&unit_type) {
                    acc.1.extend(impl_function(
                        &method,
                        &format_module,
                        &api_path,
                        &method_name,
                        &api_params,
                        &req_body_type,
                        res_body_type,
                        &unit_type,
                    ));
                }
            }
            OperationEnum::put(put) => {
                let fields = &put.fields;

                let method = parse_str::<syn::Ident>("put").unwrap();
                let mut method_name = Ident::new("ident", Span::call_site());
                let mut api_path = LitStr::new("lit", Span::call_site());
                let mut req_body_type = Type::Verbatim(TS2::new());
                let mut res_body_type = &unit_type;
                let mut api_params = TS2::new();
                let mut format_name = Ident::new("ident", Span::call_site());
                let mut format_module = Ident::new("ident", Span::call_site());

                fields.iter().for_each(|field| match field {
                    PutFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), Span::call_site());
                    }
                    PutFieldEnum::path(path) => {
                        let path_info = extract_params_from_path(&path.value);

                        api_path = path_info.1;
                        api_params = path_info.0;
                    }

                    PutFieldEnum::req_body(req_body) => {
                        req_body_type = req_body.value.clone();
                    }

                    PutFieldEnum::res_body(res_body) => {
                        res_body_type = &res_body.value;
                    }

                    PutFieldEnum::format(format) => {
                        let format_value = format.value.value();
                        let title_format_value = format_value.titlecase();
                        format_name = format_ident!("{}", format_value);
                        format_module = format_ident!("{}Body", title_format_value);
                    }
                });

                if acc.0.is_empty() {
                    acc.0.extend(quote! {
                        use deboa_extras::http::serde::#format_name::{#format_module};
                    });
                }

                acc.1.extend(impl_function(
                    &method,
                    &format_module,
                    &api_path,
                    &method_name,
                    &api_params,
                    &req_body_type,
                    res_body_type,
                    &unit_type,
                ));
            }
            OperationEnum::patch(patch) => {
                let fields = &patch.fields;

                let method = parse_str::<syn::Ident>("patch").unwrap();
                let mut method_name = Ident::new("ident", Span::call_site());
                let mut api_path = LitStr::new("lit", Span::call_site());
                let mut api_params = TS2::new();
                let mut req_body_type = Type::Verbatim(TS2::new());
                let mut res_body_type = &unit_type;
                let mut format_name = Ident::new("ident", Span::call_site());
                let mut format_module = Ident::new("ident", Span::call_site());

                fields.iter().for_each(|field| match field {
                    PatchFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), Span::call_site());
                    }

                    PatchFieldEnum::path(path) => {
                        let path_info = extract_params_from_path(&path.value);

                        api_path = path_info.1;
                        api_params = path_info.0;
                    }

                    PatchFieldEnum::req_body(req_body) => {
                        req_body_type = req_body.value.clone();
                    }

                    PatchFieldEnum::res_body(res_body) => {
                        res_body_type = &res_body.value;
                    }

                    PatchFieldEnum::format(format) => {
                        let format_value = format.value.value();
                        let title_format_value = format_value.titlecase();
                        format_name = format_ident!("{}", format_value);
                        format_module = format_ident!("{}Body", title_format_value);
                    }
                });

                if acc.0.is_empty() {
                    acc.0.extend(quote! {
                        use deboa_extras::http::serde::#format_name::{#format_module};
                    });
                }

                acc.1.extend(impl_function(
                    &method,
                    &format_module,
                    &api_path,
                    &method_name,
                    &api_params,
                    &req_body_type,
                    res_body_type,
                    &unit_type,
                ));
            }
            OperationEnum::delete(delete) => {
                let fields = &delete.fields;

                let method = parse_str::<syn::Ident>("delete").unwrap();
                let mut method_name = Ident::new("ident", Span::call_site());
                let mut api_path = LitStr::new("lit", Span::call_site());
                let mut api_params = TS2::new();

                fields.iter().for_each(|field| match field {
                    DeleteFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), Span::call_site());
                    }
                    DeleteFieldEnum::path(path) => {
                        let path_info = extract_params_from_path(&path.value);

                        api_path = path_info.1;
                        api_params = path_info.0;
                    }
                });

                acc.0.extend(quote! {});

                acc.1.extend(quote! {
                    pub async fn #method_name(&mut self, #api_params) -> Result<(), DeboaError> {
                        self.api.#method(format!(#api_path).as_ref())?.go(&mut self.api.client()).await?;
                        Ok(())
                    }
                });
            }
        }

        acc
    });

    let ts = quote! {
        use vamo::Vamo as Client;
        use deboa::{response::DeboaResponse};
        #imports

        pub struct #struct_name {
            api: Client
        }

        impl #struct_name {
            pub fn new(api: Vamo) -> Self {
                Self {
                    api
                }
            }

            #struct_impl
        }
    };

    TokenStream::from(ts)
}
