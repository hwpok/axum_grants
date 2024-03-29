use proc_macro::TokenStream;
use crate::grants::grants::expand_protect;

mod grants;

#[proc_macro_attribute]
pub fn protect(attr: TokenStream, input: TokenStream) -> TokenStream {
    expand_protect(attr, input, false)
}

#[proc_macro_attribute]
pub fn protect_diy(attr: TokenStream, input: TokenStream) -> TokenStream {
    expand_protect(attr, input, false)
}
