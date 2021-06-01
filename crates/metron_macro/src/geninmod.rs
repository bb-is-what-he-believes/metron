use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemStruct, LitInt, LitStr};

pub fn geninmod(item: TokenStream) -> TokenStream {
    // let attr = dbg!(attr);
    let item = dbg!(item);
    let expanded = quote! {
        // item
    };
    expanded.into()
}