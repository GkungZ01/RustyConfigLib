use proc_macro::TokenStream;

#[proc_macro_derive(Config, attributes(category, option))]
pub fn derive_config(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
