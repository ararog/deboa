extern crate proc_macro;
use crate::bora::{
    parser::{
        api::{BoraApi, OperationEnum},
        operations::{
            delete::{DeleteFieldEnum, DeleteStruct},
            get::{GetFieldEnum, GetStruct},
            patch::{PatchFieldEnum, PatchStruct},
            post::{PostFieldEnum, PostStruct},
            put::{PutFieldEnum, PutStruct},
        },
    },
    token::utils::extract_params_from_path,
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, parse_str, punctuated::Punctuated, token::Paren, Data, DeriveInput, Ident,
    LitStr, Type, TypeTuple, Visibility,
};
use titlecase::titlecase;

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
            pub async fn #method_name(&mut self, #api_params body: #req_body_type) -> VamoResult<#res_body_type> {
                self.api
                    .#deboa_method(format!(#api_path).as_ref())
                    .send()
                    .await?;
                Ok(())
            }
        }
    } else {
        quote! {
            pub async fn #method_name(&mut self, #api_params body: #req_body_type) -> VamoResult<#res_body_type> {
                self.api
                    .#deboa_method(format!(#api_path).as_ref())
                    .body_as(#format_module, body)?
                    .send()
                    .await?
                    .body_as(#format_module)
                    .await
            }
        }
    }
}

fn get_operation(get: &GetStruct, acc: &mut (&mut TS2, &mut TS2)) {
    let fields = &get.fields;

    let method = parse_str::<syn::Ident>("get").unwrap();
    let mut method_name = Ident::new("ident", Span::call_site());
    let mut api_path = LitStr::new("lit", Span::call_site());
    let mut res_body_type = Type::Verbatim(TS2::new());
    let mut api_params = TS2::new();
    let mut format_name = Ident::new("ident", Span::call_site());
    let mut format_module = Ident::new("ident", Span::call_site());

    fields
        .iter()
        .for_each(|field| match field {
            GetFieldEnum::name(name) => {
                method_name = Ident::new(
                    name.value
                        .value()
                        .as_str(),
                    Span::call_site(),
                );
            }
            GetFieldEnum::path(path) => {
                let path_info = extract_params_from_path(&path.value);
                api_path = path_info.1;
                api_params = path_info.0;
            }
            GetFieldEnum::res_body(res_body) => {
                res_body_type = res_body
                    .value
                    .clone();
            }
            GetFieldEnum::format(format) => {
                let format_value = format.value.value();
                format_name = format_ident!("{}", format_value);
                format_module = format_ident!("{}Body", titlecase(&format_value));
            }
        });

    if acc.0.is_empty() {
        acc.0
            .extend(quote! {
                use deboa_extras::http::serde::#format_name::{#format_module};
            });
    }

    acc.1
        .extend(quote! {
            pub async fn #method_name(&mut self, #api_params) -> VamoResult<#res_body_type> {
                self.api
                    .#method(format!(#api_path).as_ref())
                    .send()
                    .await?
                    .body_as(#format_module)
                    .await
            }
        });
}

fn post_operation(post: &PostStruct, acc: &mut (&mut TS2, &mut TS2)) {
    let fields = &post.fields;

    let unit_type =
        Type::Tuple(TypeTuple { paren_token: Paren::default(), elems: Punctuated::new() });

    let method = parse_str::<syn::Ident>("post").unwrap();
    let mut method_name = Ident::new("ident", Span::call_site());
    let mut api_path = LitStr::new("lit", Span::call_site());
    let mut req_body_type = Type::Verbatim(TS2::new());
    let mut res_body_type = &unit_type;
    let mut api_params = TS2::new();
    let mut format_name = Ident::new("ident", Span::call_site());
    let mut format_module = Ident::new("ident", Span::call_site());

    fields
        .iter()
        .for_each(|field| match field {
            PostFieldEnum::name(name) => {
                method_name = Ident::new(
                    name.value
                        .value()
                        .as_str(),
                    Span::call_site(),
                );
            }
            PostFieldEnum::path(path) => {
                let path_info = extract_params_from_path(&path.value);

                api_path = path_info.1;
                api_params = path_info.0;
            }
            PostFieldEnum::req_body(req_body) => {
                req_body_type = req_body
                    .value
                    .clone();
            }
            PostFieldEnum::res_body(res_body) => {
                res_body_type = &res_body.value;
            }
            PostFieldEnum::format(format) => {
                let format_value = format.value.value();
                let title_format_value = titlecase(&format_value);
                format_name = format_ident!("{}", format_value);
                format_module = format_ident!("{}Body", title_format_value);
            }
        });

    if acc.0.is_empty() {
        acc.0
            .extend(quote! {
                use deboa_extras::http::serde::#format_name::{#format_module};
            });
    }

    acc.1
        .extend(impl_function(
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

fn put_operation(put: &PutStruct, acc: &mut (&mut TS2, &mut TS2)) {
    let fields = &put.fields;

    let unit_type =
        Type::Tuple(TypeTuple { paren_token: Paren::default(), elems: Punctuated::new() });

    let method = parse_str::<syn::Ident>("put").unwrap();
    let mut method_name = Ident::new("ident", Span::call_site());
    let mut api_path = LitStr::new("lit", Span::call_site());
    let mut req_body_type = Type::Verbatim(TS2::new());
    let mut res_body_type = &unit_type;
    let mut api_params = TS2::new();
    let mut format_name = Ident::new("ident", Span::call_site());
    let mut format_module = Ident::new("ident", Span::call_site());

    fields
        .iter()
        .for_each(|field| match field {
            PutFieldEnum::name(name) => {
                method_name = Ident::new(
                    name.value
                        .value()
                        .as_str(),
                    Span::call_site(),
                );
            }
            PutFieldEnum::path(path) => {
                let path_info = extract_params_from_path(&path.value);
                api_path = path_info.1;
                api_params = path_info.0;
            }

            PutFieldEnum::req_body(req_body) => {
                req_body_type = req_body
                    .value
                    .clone();
            }

            PutFieldEnum::res_body(res_body) => {
                res_body_type = &res_body.value;
            }

            PutFieldEnum::format(format) => {
                let format_value = format.value.value();
                let title_format_value = titlecase(&format_value);
                format_name = format_ident!("{}", format_value);
                format_module = format_ident!("{}Body", title_format_value);
            }
        });

    if acc.0.is_empty() {
        acc.0
            .extend(quote! {
                use deboa_extras::http::serde::#format_name::{#format_module};
            });
    }

    acc.1
        .extend(impl_function(
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

fn patch_operation(patch: &PatchStruct, acc: &mut (&mut TS2, &mut TS2)) {
    let fields = &patch.fields;

    let unit_type =
        Type::Tuple(TypeTuple { paren_token: Paren::default(), elems: Punctuated::new() });

    let method = parse_str::<syn::Ident>("patch").unwrap();
    let mut method_name = Ident::new("ident", Span::call_site());
    let mut api_path = LitStr::new("lit", Span::call_site());
    let mut api_params = TS2::new();
    let mut req_body_type = Type::Verbatim(TS2::new());
    let mut res_body_type = &unit_type;
    let mut format_name = Ident::new("ident", Span::call_site());
    let mut format_module = Ident::new("ident", Span::call_site());

    fields
        .iter()
        .for_each(|field| match field {
            PatchFieldEnum::name(name) => {
                method_name = Ident::new(
                    name.value
                        .value()
                        .as_str(),
                    Span::call_site(),
                );
            }

            PatchFieldEnum::path(path) => {
                let path_info = extract_params_from_path(&path.value);
                api_path = path_info.1;
                api_params = path_info.0;
            }

            PatchFieldEnum::req_body(req_body) => {
                req_body_type = req_body
                    .value
                    .clone();
            }

            PatchFieldEnum::res_body(res_body) => {
                res_body_type = &res_body.value;
            }

            PatchFieldEnum::format(format) => {
                let format_value = format.value.value();
                let title_format_value = titlecase(&format_value);
                format_name = format_ident!("{}", format_value);
                format_module = format_ident!("{}Body", title_format_value);
            }
        });

    if acc.0.is_empty() {
        acc.0
            .extend(quote! {
                use deboa_extras::http::serde::#format_name::{#format_module};
            });
    }

    acc.1
        .extend(impl_function(
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

fn delete_operation(delete: &DeleteStruct, acc: &mut (&mut TS2, &mut TS2)) {
    let fields = &delete.fields;

    let method = parse_str::<syn::Ident>("delete").unwrap();
    let mut method_name = Ident::new("ident", Span::call_site());
    let mut api_path = LitStr::new("lit", Span::call_site());
    let mut api_params = TS2::new();

    fields
        .iter()
        .for_each(|field| match field {
            DeleteFieldEnum::name(name) => {
                method_name = Ident::new(
                    name.value
                        .value()
                        .as_str(),
                    Span::call_site(),
                );
            }
            DeleteFieldEnum::path(path) => {
                let path_info = extract_params_from_path(&path.value);
                api_path = path_info.1;
                api_params = path_info.0;
            }
        });

    acc.0
        .extend(quote! {});

    acc.1
        .extend(quote! {
            pub async fn #method_name(&mut self, #api_params) -> VamoResult<()> {
                self
                    .api
                    .#method(&format!(#api_path))
                    .send()
                    .await?;
                Ok(())
            }
        });
}

pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    let attr = TS2::from(attr);

    let parse_attr = attr.clone().into();
    let root = parse_macro_input!(parse_attr as BoraApi);

    let name = if matches!(item.vis, Visibility::Public(_)) && matches!(item.data, Data::Struct(_))
    {
        item.ident
            .to_string()
    } else {
        panic!("expected public struct");
    };

    let struct_name = parse_str::<syn::Ident>(name.as_str()).unwrap();

    let mut imports = TS2::new();
    let mut struct_impl = TS2::new();

    root.operations
        .iter()
        .fold((&mut imports, &mut struct_impl), |mut acc, op| {
            match op {
                OperationEnum::get(get) => get_operation(get, &mut acc),
                OperationEnum::post(post) => post_operation(post, &mut acc),
                OperationEnum::put(put) => put_operation(put, &mut acc),
                OperationEnum::patch(patch) => patch_operation(patch, &mut acc),
                OperationEnum::delete(delete) => delete_operation(delete, &mut acc),
            }

            acc
        });

    let ts = quote! {
        use deboa::{response::DeboaResponse, Result as VamoResult};
        #imports

        pub struct #struct_name<C> {
            api: vamo::Vamo<C>
        }

        impl<C> #struct_name<C>
        where
            C: deboa::HttpClient + Default
        {
            pub fn new(api: vamo::Vamo<C>) -> Self {
                Self {
                    api
                }
            }

            #struct_impl
        }
    };

    TokenStream::from(ts)
}
