use proc_macro::TokenStream;

mod bora;
mod parser;
mod token;

use crate::bora::api::bora as bora_macro;

#[proc_macro_attribute]
pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    bora_macro(attr, item)
}
