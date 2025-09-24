use proc_macro::TokenStream;

pub fn patch(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    TokenStream::new()
}
