extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{Ident, LitStr, parse_macro_input};

use parser::BoraApi;

use crate::parser::{GetFieldEnum, OperationEnum};

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

    root.operations
        .iter()
        .fold((&mut trait_functions, &mut trait_impl), |acc, op| {
            match op {
                OperationEnum::get(get) => {
                    let fields = &get.fields;

                    let method = syn::parse_str::<syn::Ident>("get").unwrap();
                    let mut method_name = Ident::new("ident", proc_macro2::Span::call_site());
                    let mut api_path = LitStr::new("lit", proc_macro2::Span::call_site());
                    let mut target_type = Ident::new("ident", proc_macro2::Span::call_site());

                    fields.iter().for_each(|field| match field {
                        GetFieldEnum::name(name) => {
                            method_name = Ident::new(
                                name.value.value().as_str(),
                                proc_macro2::Span::call_site(),
                            );
                        }
                        GetFieldEnum::path(path) => {
                            api_path = path.value.clone();
                        }
                        GetFieldEnum::target(target) => {
                            target_type = target.value.clone();
                        }
                    });

                    acc.0.extend(quote! {
                        async fn #method_name(&self) -> Result<#target_type, DeboaError>;
                    });

                    acc.1.extend(quote! {
                        async fn #method_name(&self) -> Result<#target_type, DeboaError> {
                            self.api.#method(#api_path).await?.json::<#target_type>().await
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
