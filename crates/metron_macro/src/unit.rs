// use proc_macro::TokenStream;
// use proc_macro2::Span;
// use quote::quote;
// use syn::group::Braces;
// use syn::parse::{Parse, ParseBuffer, ParseStream, Result};
// use syn::punctuated::Punctuated;
// use syn::token::{Brace, FatArrow, Underscore};
// use syn::{
//     parse_macro_input, Attribute, BinOp, Block, Fields, Generics, Ident, ItemStruct, LitFloat,
//     LitInt, LitStr, TypePath, Visibility,
// };
// use convert_case::{Case, Casing};
// use crate::util::ident_of;
//
// pub fn unit(input: TokenStream) -> TokenStream {
//     // let input = dbg!(input);
//     let unit = parse_macro_input!(input as Unit);
//     let str_pascal = unit.ident.to_string().to_case(Case::Pascal);
//     let str_snake = str_pascal.to_case(Case::Snake);
//     let ident_pascal = ident_of(&str_pascal);
//     let ident_snake = ident_of(&str_snake);
//     let pac_measure = quote! {metron_core::measure};
//     let pac_unit = quote! {metron_core::unit};
//     let unit_def = {
//         quote!{
//             pub mod #ident_snake{
//                 pub struct #ident_pascal;
//                 impl #pac_unit::Unit for #ident_pascal{
//                     fn symbol(_fmt: &metron_core::fmt::Formatter) -> &'static str {
//                         #str_pascal
//                     }
//                 }
//             }
//         }
//     };
//     let type_def = {
//         quote!{
//             pub type #ident_pascal<N> = #pac_measure::Measure<N, self::#ident_snake::#ident_pascal>;
//         }
//     };
//     dbg!(type_def.to_string());
//     let expanded = quote! {
//         #unit_def
//         #type_def
//     };
//     dbg!(expanded.to_string());
//     // dbg!(expanded).into()
//     expanded.into()
// }
// mod kw {
//     syn::custom_keyword!(unit);
//     syn::custom_keyword!(from);
//     syn::custom_keyword!(exp);
//     syn::custom_keyword!(disp);
// }
// struct Unit {
//     attrs: Vec<Attribute>,
//     vis: Visibility,
//     unit_kw: kw::unit,
//     ident: Ident,
//     generics: Generics,
//     brace: Brace,
//     fields: Punctuated<UnitFields, Token![,]>,
// }
// impl Parse for Unit {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("Unit");
//         let mut attrs = input.call(Attribute::parse_outer)?;
//         let vis = input.parse::<Visibility>()?;
//         let unit_kw = input.parse::<kw::unit>()?;
//         let ident = input.parse::<Ident>()?;
//         let generics = input.parse::<Generics>()?;
//
//         let where_clause = input.parse()?;
//
//         let content;
//         let brace = braced!(content in input);
//         let fields = content.parse_terminated(UnitFields::parse)?;
//         Ok(Unit {
//             attrs,
//             vis,
//             unit_kw,
//             ident,
//             generics: Generics {
//                 where_clause,
//                 ..generics
//             },
//             brace,
//             fields,
//         })
//     }
// }
//
// enum UnitFields {
//     From(From),
//     Exp(Exp),
//     Disp(Disp),
// }
// impl Parse for UnitFields {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("UnitFields");
//         let lookahead = input.lookahead1();
//         if lookahead.peek(kw::from) {
//             Ok(Self::From(input.parse()?))
//         } else if lookahead.peek(kw::exp) {
//             Ok(Self::Exp(input.parse()?))
//         } else if lookahead.peek(kw::disp) {
//             Ok(Self::Disp(input.parse()?))
//         } else {
//             Err(lookahead.error())
//         }
//     }
// }
//
// struct UnitField<KW, SIN, MUL> {
//     kw: KW,
//     colon: Token![:],
//     body: UnitFieldBody<SIN, MUL>,
// }
// impl<KW: Parse, SIN: Parse, MUL: Parse> Parse for UnitField<KW, SIN, MUL> {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("UnitField");
//         Ok(Self {
//             kw: input.parse()?,
//             colon: input.parse()?,
//             body: input.parse()?,
//         })
//     }
// }
// enum UnitFieldBody<SIN, MUL> {
//     Single(SIN),
//     Multi(UnitFieldMultiBody<MUL>),
// }
//
// impl<SIN: Parse, MUL: Parse> Parse for UnitFieldBody<SIN, MUL> {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("UnitFieldBody");
//         let lookahead = input.lookahead1();
//         if lookahead.peek(Brace) {
//             Ok(Self::Multi(input.parse()?))
//         } else {
//             Ok(Self::Single(input.parse()?))
//         }
//     }
// }
// struct UnitFieldMultiBody<MUL> {
//     brace: Brace,
//     defs: Punctuated<MUL, Token![,]>,
// }
// impl<MUL: Parse> Parse for UnitFieldMultiBody<MUL> {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("UnitFieldMultiBody");
//         let content;
//         Ok(Self {
//             brace: braced!(content in input),
//             defs: content.parse_terminated(MUL::parse)?,
//         })
//     }
// }
// type From = UnitField<kw::from, FromDef, FromDef>;
// struct FromDef {
//     type_path: TypePath,
//     expr: FromDefExpr,
// }
// impl Parse for FromDef {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("FromDef");
//         Ok(Self {
//             type_path: input.parse()?,
//             expr: input.parse()?,
//         })
//     }
// }
// struct FromDefExpr {
//     op: BinOp,
//     lit: LitFloat,
// }
//
// impl Parse for FromDefExpr {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("FromDefExpr");
//         Ok(Self {
//             op: input.parse()?,
//             lit: input.parse()?,
//         })
//     }
// }
// type Exp = UnitField<kw::exp, ExpDef, ExpDef>;
// struct ExpDef {
//     type_path: TypePath,
// }
// impl Parse for ExpDef {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("ExpDef");
//         Ok(Self {
//             type_path: input.parse()?,
//         })
//     }
// }
// type Disp = UnitField<kw::disp, DispSingleDef, DispMultiDef>;
// struct DispSingleDef {
//     lit: LitStr,
// }
// impl Parse for DispSingleDef {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("DispSingleDef");
//         Ok(Self {
//             lit: input.parse()?,
//         })
//     }
// }
// struct DispMultiDef {
//     underscore_token: DispMultiDefPat,
//     fat_arrow_token: FatArrow,
//     lit: LitStr,
// }
// impl Parse for DispMultiDef {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("DispMultiDef");
//         Ok(Self {
//             underscore_token: input.parse()?,
//             fat_arrow_token: input.parse()?,
//             lit: input.parse()?,
//         })
//     }
// }
//
// enum DispMultiDefPat {
//     Wild(Underscore),
//     Type(TypePath),
// }
//
// impl Parse for DispMultiDefPat {
//     fn parse(input: ParseStream) -> Result<Self> {
//         dbg!("DispMultiDefPat");
//         let lookahead = input.lookahead1();
//         if lookahead.peek(Underscore) {
//             Ok(Self::Wild(input.parse()?))
//         } else {
//             Ok(Self::Type(input.parse()?))
//         }
//     }
// }
