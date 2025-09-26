use proc_macro::TokenStream;

mod bora;
mod parser;
mod token;

use crate::bora::api::bora as bora_macro;
use crate::bora::delete::delete as delete_macro;
use crate::bora::get::get as get_macro;
use crate::bora::patch::patch as patch_macro;
use crate::bora::post::post as post_macro;
use crate::bora::put::put as put_macro;

#[proc_macro_attribute]
pub fn bora(attr: TokenStream, item: TokenStream) -> TokenStream {
    bora_macro(attr, item)
}

#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    get_macro(attr, item)
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    post_macro(attr, item)
}

#[proc_macro_attribute]
pub fn put(attr: TokenStream, item: TokenStream) -> TokenStream {
    put_macro(attr, item)
}

#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    delete_macro(attr, item)
}

#[proc_macro_attribute]
pub fn patch(attr: TokenStream, item: TokenStream) -> TokenStream {
    patch_macro(attr, item)
}
