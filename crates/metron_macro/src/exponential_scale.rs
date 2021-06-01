use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemStruct, LitInt, LitStr};
use crate::util::ident_of;

pub fn exponential_scale(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (symbol, base, exponent) = {
        let ExponentialScale { items } = parse_macro_input!(attr as ExponentialScale);
        let mut symbol = None;
        let mut base = None;
        let mut exponent = None;
        for item in items {
            match item {
                Item::Symbol(item_symbol) => symbol = Some(item_symbol),
                Item::Base(item_base) => base = Some(item_base),
                Item::Exponent(item_exponent) => exponent = Some(item_exponent),
            }
        }
        (symbol.unwrap(), base.unwrap(), exponent.unwrap())
    };
    let ItemStruct {
        ident,
        ..
    } = parse_macro_input!(item as ItemStruct);
    let pac_exp = quote! {metron_core::unit::scale::exp};
    let pac_fmt = quote! {metron_core::fmt};
    let expanded = quote! {
        struct #ident;
        impl #pac_exp::Scale for #ident {
            fn symbol(_fmt: &#pac_fmt::Formatter) -> &'static str {
                #symbol
            }
        }
        impl #pac_exp::ExponentialScale for #ident {
            type ScaleBase     = #pac_exp::#base;
            type ScaleExponent = #pac_exp::#exponent;
        }
    };
    expanded.into()
}

struct ExponentialScale {
    items: Punctuated<Item, Token![,]>,
}
impl Parse for ExponentialScale {
    fn parse(input: ParseStream) -> Result<Self> {
        let items = input.parse_terminated(Item::parse)?;
        Ok(Self { items })
    }
}
mod kw {
    syn::custom_keyword!(symbol);
    syn::custom_keyword!(base);
    syn::custom_keyword!(exponent);
}
enum Item {
    Symbol(LitStr),
    Base(Ident),
    Exponent(Ident),
}
impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::symbol) {
            let sym = parse_symbol(input)?;
            Ok(Item::Symbol(sym))
        } else if lookahead.peek(kw::base) {
            let base = parse_base(input)?;
            Ok(Item::Base(base))
        } else if lookahead.peek(kw::exponent) {
            let exp = parse_exponent(input)?;
            Ok(Item::Exponent(exp))
        } else {
            Err(lookahead.error())
        }
    }
}
fn parse_symbol(input: ParseStream) -> Result<LitStr> {
    input.parse::<kw::symbol>()?;
    input.parse::<Token![=]>()?;
    input.parse()
}
fn parse_base(input: ParseStream) -> Result<Ident> {
    input.parse::<kw::base>()?;
    input.parse::<Token![=]>()?;
    let num_base = input.parse::<LitInt>()?.base10_parse::<u8>()?;
    match num_base {
        10 => Ok(ident_of("ScaleBase10")),
        2 => Ok(ident_of("ScaleBase2")),
        _ => Err(input.error("unsupported")),
    }
}
fn parse_exponent(input: ParseStream) -> Result<Ident> {
    input.parse::<kw::exponent>()?;
    input.parse::<Token![=]>()?;
    let num_exponent = input.parse::<LitInt>()?.base10_parse::<i8>()?;
    match num_exponent {
        3 => Ok(ident_of("ScaleExponentP3")),
        0 => Ok(ident_of("ScaleExponent0")),
        -3 => Ok(ident_of("ScaleExponentM3")),
        _ => Err(input.error("unsupported")),
    }
}

