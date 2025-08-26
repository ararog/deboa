extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{Ident, LitStr, parse_macro_input};

use parser::BoraApi;

use crate::parser::{DeleteFieldEnum, GetFieldEnum, OperationEnum, PatchFieldEnum, PostFieldEnum, PutFieldEnum};

mod parser;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let attr = proc_macro2::TokenStream::from(attr);

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

    let struct_name = syn::parse_str::<syn::Ident>(name.as_str()).unwrap();

    let mut trait_functions = proc_macro2::TokenStream::new();
    let mut trait_impl = proc_macro2::TokenStream::new();

    root.operations.iter().fold((&mut trait_functions, &mut trait_impl), |acc, op| {
        match op {
            OperationEnum::get(get) => {
                let fields = &get.fields;

                let method = syn::parse_str::<syn::Ident>("get").unwrap();
                let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                let mut target_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                let mut api_params = proc_macro2::TokenStream::new();

                fields.iter().for_each(|field| match field {
                    GetFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                    }
                    GetFieldEnum::path(path) => {
                        let path = &path.value;

                        let raw_path = path.value();
                        let params = regex::Regex::new(r"<(\w*:\w*)>")
                            .unwrap()
                            .captures(&raw_path)
                            .map(|m| m.get(1).unwrap().as_str())
                            .into_iter()
                            .collect::<Vec<_>>();

                        api_params = params.clone().into_iter().fold(proc_macro2::TokenStream::new(), |mut acc, param| {
                            let pair = param.split(':').collect::<Vec<_>>();
                            let param = syn::parse_str::<syn::Ident>(pair[0]).unwrap();
                            let param_type = syn::parse_str::<syn::Type>(pair[1]).unwrap();
                            acc.extend(quote! {
                                #param: #param_type,
                            });
                            acc
                        });

                        let new_path = regex::Regex::new(r"<(\w*):\w*>").unwrap().replace_all(&raw_path, "{$1}");

                        api_path = LitStr::new(&new_path, proc_macro2::Span::call_site());
                    }
                    GetFieldEnum::res_body(res_body) => {
                        target_type = res_body.value.clone();
                    }
                });

                acc.0.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError>;
                });

                acc.1.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError> {
                        self.api.#method(format!(#api_path).as_ref()).await?.json::<#target_type>().await
                    }
                });
            }
            OperationEnum::post(post) => {
                let fields = &post.fields;

                let method = syn::parse_str::<syn::Ident>("post").unwrap();
                let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                let mut target_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                let mut api_params = proc_macro2::TokenStream::new();

                fields.iter().for_each(|field| match field {
                    PostFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                    }
                    PostFieldEnum::path(path) => {
                        let path = &path.value;

                        let raw_path = path.value();
                        let params = regex::Regex::new(r"<(\w*):\w*>")
                            .unwrap()
                            .captures(&raw_path)
                            .map(|m| m.get(1).unwrap().as_str())
                            .into_iter()
                            .collect::<Vec<_>>();

                        api_params = params.clone().into_iter().fold(proc_macro2::TokenStream::new(), |mut acc, param| {
                            let pair = param.split(':').collect::<Vec<_>>();
                            let param = syn::parse_str::<syn::Ident>(pair[0]).unwrap();
                            let param_type = syn::parse_str::<syn::Type>(pair[1]).unwrap();
                            acc.extend(quote! {
                                #param: #param_type,
                            });
                            acc
                        });

                        let new_path = regex::Regex::new(r"<(\w*):\w*>").unwrap().replace_all(&raw_path, "{$1}");

                        api_path = LitStr::new(&new_path, proc_macro2::Span::call_site());
                    }
                    PostFieldEnum::req_body(req_body) => {
                        target_type = req_body.value.clone();
                    }
                });

                acc.0.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError>;
                });

                acc.1.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError> {
                        self.api.#method(format!(#api_path).as_ref()).await?.json::<#target_type>().await
                    }
                });
            }
            OperationEnum::put(put) => {
                let fields = &put.fields;

                let method = syn::parse_str::<syn::Ident>("put").unwrap();
                let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                let mut target_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                let mut api_params = proc_macro2::TokenStream::new();

                fields.iter().for_each(|field| match field {
                    PutFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                    }
                    PutFieldEnum::path(path) => {
                        let path = &path.value;

                        let raw_path = path.value();
                        let params = regex::Regex::new(r"<(\w*):\w*>")
                            .unwrap()
                            .captures(&raw_path)
                            .map(|m| m.get(1).unwrap().as_str())
                            .into_iter()
                            .collect::<Vec<_>>();

                        api_params = params.clone().into_iter().fold(proc_macro2::TokenStream::new(), |mut acc, param| {
                            let pair = param.split(':').collect::<Vec<_>>();
                            let param = syn::parse_str::<syn::Ident>(pair[0]).unwrap();
                            let param_type = syn::parse_str::<syn::Type>(pair[1]).unwrap();
                            acc.extend(quote! {
                                #param: #param_type,
                            });
                            acc
                        });

                        let new_path = regex::Regex::new(r"<(\w*):\w*>").unwrap().replace_all(&raw_path, "{$1}");

                        api_path = LitStr::new(&new_path, proc_macro2::Span::call_site());
                    }
                    PutFieldEnum::req_body(req_body) => {
                        target_type = req_body.value.clone();
                    }
                });

                acc.0.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError>;
                });

                acc.1.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError> {
                        self.api.#method(format!(#api_path).as_ref()).await?.json::<#target_type>().await
                    }
                });
            }
            OperationEnum::delete(delete) => {
                let fields = &delete.fields;

                let method = syn::parse_str::<syn::Ident>("delete").unwrap();
                let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                let mut api_params = proc_macro2::TokenStream::new();

                fields.iter().for_each(|field| match field {
                    DeleteFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                    }
                    DeleteFieldEnum::path(path) => {
                        let path = &path.value;

                        let raw_path = path.value();
                        let params = regex::Regex::new(r"<(\w*):\w*>")
                            .unwrap()
                            .captures(&raw_path)
                            .map(|m| m.get(1).unwrap().as_str())
                            .into_iter()
                            .collect::<Vec<_>>();

                        api_params = params.clone().into_iter().fold(proc_macro2::TokenStream::new(), |mut acc, param| {
                            let pair = param.split(':').collect::<Vec<_>>();
                            let param = syn::parse_str::<syn::Ident>(pair[0]).unwrap();
                            let param_type = syn::parse_str::<syn::Type>(pair[1]).unwrap();
                            acc.extend(quote! {
                                #param: #param_type,
                            });
                            acc
                        });

                        let new_path = regex::Regex::new(r"<(\w*):\w*>").unwrap().replace_all(&raw_path, "{$1}");

                        api_path = LitStr::new(&new_path, proc_macro2::Span::call_site());
                    }
                });

                acc.0.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<(), DeboaError>;
                });

                acc.1.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<(), DeboaError> {
                        self.api.#method(format!(#api_path).as_ref()).await
                    }
                });
            }
            OperationEnum::patch(patch) => {
                let fields = &patch.fields;

                let method = syn::parse_str::<syn::Ident>("patch").unwrap();
                let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                let mut api_params = proc_macro2::TokenStream::new();
                let mut target_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                fields.iter().for_each(|field| match field {
                    PatchFieldEnum::name(name) => {
                        method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                    }

                    PatchFieldEnum::path(path) => {
                        let path = &path.value;

                        let raw_path = path.value();
                        let params = regex::Regex::new(r"<(\w*):\w*>")
                            .unwrap()
                            .captures(&raw_path)
                            .map(|m| m.get(1).unwrap().as_str())
                            .into_iter()
                            .collect::<Vec<_>>();

                        api_params = params.clone().into_iter().fold(proc_macro2::TokenStream::new(), |mut acc, param| {
                            let pair = param.split(':').collect::<Vec<_>>();
                            let param = syn::parse_str::<syn::Ident>(pair[0]).unwrap();
                            let param_type = syn::parse_str::<syn::Type>(pair[1]).unwrap();
                            acc.extend(quote! {
                                #param: #param_type,
                            });
                            acc
                        });

                        let new_path = regex::Regex::new(r"<(\w*):\w*>").unwrap().replace_all(&raw_path, "{$1}");

                        api_path = LitStr::new(&new_path, proc_macro2::Span::call_site());
                    }

                    PatchFieldEnum::req_body(req_body) => {
                        target_type = req_body.value.clone();
                    }
                });

                acc.0.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError>;
                });

                acc.1.extend(quote! {
                    async fn #method_name(&self, #api_params) -> Result<#target_type, DeboaError> {
                        self.api.#method(format!(#api_path).as_ref()).await?.json::<#target_type>().await
                    }
                });
            }
        }

        acc
    });

    let ts = quote! {
        use deboa::{Deboa, DeboaError};

        pub struct #struct_name {
            api: Deboa
        }

        pub trait Service {
            fn new(api: Deboa) -> Self;
            #trait_functions
        }

        impl Service for #struct_name {
            fn new(api: Deboa) -> Self {
                Self {
                    api
                }
            }

            #trait_impl
        }
    };

    TokenStream::from(ts)
}
