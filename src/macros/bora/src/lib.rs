extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;

#[derive(Default, Debug)]
struct Request {
    pub method: String,
    pub path: String,
    pub target: String,
}

#[proc_macro_attribute]
pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let attr = proc_macro2::TokenStream::from(attr);

    println!("attr: {attr:?}");
    println!("item: {item:?}");

    let request = attr
        .into_iter()
        .fold(Request::default(), |mut acc, tt| match tt {
            TokenTree::Literal(lit) => {
                acc.path = lit.to_string();
                acc
            }
            TokenTree::Ident(ident) => {
                if acc.method.is_empty() {
                    acc.method = ident.to_string();
                } else {
                    acc.target = ident.to_string();
                }
                acc
            }
            _ => acc,
        });

    let mut iterator = item.into_iter();

    println!("request: {request:?}");

    let tt = iterator.next().unwrap();
    let visibility = if let TokenTree::Ident(ident) = tt {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    println!("visibility: {visibility}");
    if visibility != "pub" {
        panic!("expected to be pub");
    }


    let tt = iterator.next().unwrap();
    let type_name = if let TokenTree::Ident(ident) = tt {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    println!("type_name: {type_name}");
    if type_name != "struct" {
        panic!("expected to be struct");
    }
  
    let tt = iterator.next().unwrap();
    let name = if let TokenTree::Ident(ident) = tt {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    println!("name: {name}");
    if name.is_empty() {
        panic!("expected a name");
    }

    let path = syn::parse_str::<syn::LitStr>(request.path.as_str()).unwrap();
    let struct_name = syn::parse_str::<syn::Ident>(name.as_str()).unwrap();
    let target_type = syn::parse_str::<syn::Ident>(request.target.as_str()).unwrap();

    let ts = quote! {
        use deboa::{Deboa, DeboaError};

        pub struct #struct_name {
            api: Deboa
        }

        pub trait Service {
            fn new(api: Deboa) -> Self;
            async fn get(&self) -> Result<#target_type, DeboaError>;
        }

        impl Service for #struct_name {
            fn new(api: Deboa) -> Self {
                Self {
                    api
                }
            }

            async fn get(&self) -> Result<#target_type, DeboaError> {
                self.api.get(#path).await?.json::<#target_type>().await
            }
        }
    };

    TokenStream::from(ts)
}
