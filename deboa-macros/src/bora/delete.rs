use proc_macro::TokenStream;

pub fn delete(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
