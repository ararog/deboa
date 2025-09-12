extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2, TokenTree};
use quote::{format_ident, quote};
use regex::Regex;
use syn::{Ident, LitStr, Type, TypeTuple, parse_macro_input, parse_str, punctuated::Punctuated, token::Paren};

use parser::BoraApi;
use titlecase::Titlecase;

use crate::bora::parser::{DeleteFieldEnum, GetFieldEnum, OperationEnum, PatchFieldEnum, PostFieldEnum, PutFieldEnum};

mod parser;

fn extract_params_from_path(path: &LitStr) -> (TS2, LitStr) {
    let raw_path = path.value();
    let params = Regex::new(r"<(\w*:\w*)>")
        .unwrap()
        .captures(&raw_path)
        .map(|m| m.get(1).unwrap().as_str())
        .into_iter()
        .collect::<Vec<_>>();

    let api_params = params.clone().into_iter().fold(TS2::new(), |mut acc, param| {
        let pair = param.split(':').collect::<Vec<_>>();
        let param = parse_str::<syn::Ident>(pair[0]).unwrap();
        let param_type = parse_str::<Type>(pair[1]).unwrap();
        acc.extend(quote! {
            #param: #param_type,
        });
        acc
    });

    let new_path = Regex::new(r"<(\w*):\w*>").unwrap().replace_all(&raw_path, "{$1}");

    (api_params, LitStr::new(&new_path, Span::call_site()))
}

fn trait_function(method_name: &Ident, api_params: &TS2, req_body_type: &Type, res_body_type: &Type) -> TS2 {
    quote! {
        async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<#res_body_type, DeboaError>;
    }
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
            async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<#res_body_type, DeboaError> {
                self.api.#deboa_method(format!(#api_path).as_ref()).send_with(&mut self.api.client()).await?;
                Ok(())
            }
        }
    } else {
        quote! {
            async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<#res_body_type, DeboaError> {
                self.api.#deboa_method(format!(#api_path).as_ref()).send_with(&mut self.api.client()).set_body_as(#format_module, body)?.await?.body_as(#format_module)
            }
        }
    }
}

pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = TS2::from(item);
    let attr = TS2::from(attr);

    let parse_attr = attr.clone().into();
    let root = parse_macro_input!(parse_attr as BoraApi);

    let mut iterator = item.into_iter();

    let tt = iterator.next().unwrap();
    let visibility = if let TokenTree::Ident(ident) = tt {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    if visibility != "pub" {
        panic!("expected to be pub");
    }

    let tt = iterator.next().unwrap();
    let type_name = if let TokenTree::Ident(ident) = tt {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    if type_name != "struct" {
        panic!("expected to be struct");
    }

    let tt = iterator.next().unwrap();
    let name = if let TokenTree::Ident(ident) = tt {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    if name.is_empty() {
        panic!("expected a name");
    }

    let struct_name = parse_str::<syn::Ident>(name.as_str()).unwrap();

    let mut imports = TS2::new();
    let mut trait_functions = TS2::new();
    let mut trait_impl = TS2::new();
    let unit_type = Type::Tuple(TypeTuple {
        paren_token: Paren::default(),
        elems: Punctuated::new(),
    });

    root.operations
        .iter()
        .fold((&mut imports, &mut trait_functions, &mut trait_impl), |acc, op| {
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
                        async fn #method_name(&mut self, #api_params) -> Result<#res_body_type, DeboaError>;
                    });

                    acc.2.extend(quote! {
                        async fn #method_name(&mut self, #api_params) -> Result<#res_body_type, DeboaError> {
                            self.api.#method(format!(#api_path).as_ref()).send_with(&mut self.api.client()).await?.body_as(#format_module)
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

                    acc.1.extend(trait_function(&method_name, &api_params, &req_body_type, res_body_type));

                    if res_body_type.eq(&unit_type) {
                        acc.2.extend(impl_function(
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

                    acc.1.extend(trait_function(&method_name, &api_params, &req_body_type, res_body_type));

                    acc.2.extend(impl_function(
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

                    acc.1.extend(trait_function(&method_name, &api_params, &req_body_type, res_body_type));

                    acc.2.extend(impl_function(
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
                        async fn #method_name(&mut self, #api_params) -> Result<(), DeboaError>;
                    });

                    acc.2.extend(quote! {
                        async fn #method_name(&mut self, #api_params) -> Result<(), DeboaError> {
                            self.api.#method(format!(#api_path).as_ref()).send_with(&mut self.api.client()).await?;
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

        pub trait Service {
            fn new(api: Vamo) -> Self;
            #trait_functions
        }

        impl Service for #struct_name {
            fn new(api: Vamo) -> Self {
                Self {
                    api
                }
            }

            #trait_impl
        }
    };

    TokenStream::from(ts)
}
