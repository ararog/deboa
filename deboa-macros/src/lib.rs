use proc_macro::TokenStream;

mod bora;

use crate::bora::bora as bora_macro;

#[proc_macro_attribute]
pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    bora_macro(attr, item)
}
