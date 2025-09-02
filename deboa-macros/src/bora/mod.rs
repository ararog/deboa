extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};
use syn::{Ident, LitStr, parse_macro_input};

use parser::BoraApi;
use titlecase::Titlecase;

use crate::bora::parser::{DeleteFieldEnum, GetFieldEnum, OperationEnum, PatchFieldEnum, PostFieldEnum, PutFieldEnum};

mod parser;

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

    let mut imports = proc_macro2::TokenStream::new();
    let mut trait_functions = proc_macro2::TokenStream::new();
    let mut trait_impl = proc_macro2::TokenStream::new();

    root.operations
        .iter()
        .fold((&mut imports, &mut trait_functions, &mut trait_impl), |acc, op| {
            match op {
                OperationEnum::get(get) => {
                    let fields = &get.fields;

                    let method = syn::parse_str::<syn::Ident>("get").unwrap();
                    let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                    let mut res_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut api_params = proc_macro2::TokenStream::new();
                    let mut format_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut format_module = Ident::new("ident", proc_macro2::Span::call_site());

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
                            self.api.#method(format!(#api_path).as_ref()).await?.body_as(#format_module)
                        }
                    });
                }
                OperationEnum::post(post) => {
                    let fields = &post.fields;

                    let method = syn::parse_str::<syn::Ident>("post").unwrap();
                    let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                    let mut req_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut res_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut format_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut format_module = Ident::new("ident", proc_macro2::Span::call_site());

                    fields.iter().for_each(|field| match field {
                        PostFieldEnum::name(name) => {
                            method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                        }
                        PostFieldEnum::path(path) => {
                            api_path = path.value.clone();
                        }
                        PostFieldEnum::req_body(req_body) => {
                            req_body_type = req_body.value.clone();
                        }
                        PostFieldEnum::res_body(res_body) => {
                            res_body_type = res_body.value.clone();
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

                    acc.1.extend(quote! {
                        async fn #method_name(&mut self, body: #req_body_type) -> Result<DeboaResponse, DeboaError>;
                    });

                    acc.2.extend(quote! {
                        async fn #method_name(&mut self, body: #req_body_type) -> Result<DeboaResponse, DeboaError> {
                            self.api.set_body_as(#format_module, body)?.#method(format!(#api_path).as_ref()).await
                        }
                    });
                }
                OperationEnum::put(put) => {
                    let fields = &put.fields;

                    let method = syn::parse_str::<syn::Ident>("put").unwrap();
                    let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                    let mut req_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut res_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut api_params = proc_macro2::TokenStream::new();
                    let mut format_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut format_module = Ident::new("ident", proc_macro2::Span::call_site());

                    fields.iter().for_each(|field| match field {
                        PutFieldEnum::name(name) => {
                            method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                        }
                        PutFieldEnum::path(path) => {
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

                        PutFieldEnum::req_body(req_body) => {
                            req_body_type = req_body.value.clone();
                        }

                        PutFieldEnum::res_body(res_body) => {
                            res_body_type = res_body.value.clone();
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

                    acc.1.extend(quote! {
                        async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<(), DeboaError>;
                    });

                    acc.2.extend(quote! {
                        async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<(), DeboaError> {
                            self.api.set_body_as(#format_module, body)?.#method(format!(#api_path).as_ref()).await?;
                            Ok(())
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
                    });

                    acc.0.extend(quote! {});

                    acc.1.extend(quote! {
                        async fn #method_name(&mut self, #api_params) -> Result<(), DeboaError>;
                    });

                    acc.2.extend(quote! {
                        async fn #method_name(&mut self, #api_params) -> Result<(), DeboaError> {
                            self.api.#method(format!(#api_path).as_ref()).await?;
                            Ok(())
                        }
                    });
                }
                OperationEnum::patch(patch) => {
                    let fields = &patch.fields;

                    let method = syn::parse_str::<syn::Ident>("patch").unwrap();
                    let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                    let mut api_params = proc_macro2::TokenStream::new();
                    let mut req_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut res_body_type = syn::Type::Verbatim(proc_macro2::TokenStream::new());
                    let mut format_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut format_module = Ident::new("ident", proc_macro2::Span::call_site());

                    fields.iter().for_each(|field| match field {
                        PatchFieldEnum::name(name) => {
                            method_name = Ident::new(name.value.value().as_str(), proc_macro2::Span::call_site());
                        }

                        PatchFieldEnum::path(path) => {
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

                        PatchFieldEnum::req_body(req_body) => {
                            req_body_type = req_body.value.clone();
                        }

                        PatchFieldEnum::res_body(res_body) => {
                            res_body_type = res_body.value.clone();
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

                    acc.1.extend(quote! {
                        async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<(), DeboaError>;
                    });

                    acc.2.extend(quote! {
                        async fn #method_name(&mut self, #api_params body: #req_body_type) -> Result<(), DeboaError> {
                            self.api.set_body_as(#format_module, body)?.#method(format!(#api_path).as_ref()).await?;
                            Ok(())
                        }
                    });
                }
            }

            acc
        });

    let ts = quote! {
        use deboa::{Deboa, errors::DeboaError, response::DeboaResponse};
        #imports

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
