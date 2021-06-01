// use proc_macro::TokenStream;

// mod util;
// macro_rules! proc_macro{
//     { $name: ident } => {
//         mod $name;
//         #[proc_macro]
//         pub fn $name(item: TokenStream) -> TokenStream {
//             $name::$name(item)
//         }
//     }
// }
//
// macro_rules! proc_macro_derive{
//     { $ident:ident, $name: ident } => {
//         mod $name;
//         #[proc_macro_derive($ident)]
//         pub fn $name(item: TokenStream) -> TokenStream {
//             $name::$name(item)
//         }
//     }
// }
//
// macro_rules! proc_macro_attribute{
//     { $name:ident } =>{
//         mod $name;
//         #[proc_macro_attribute]
//         pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
//             $name::$name(attr, item)
//         }
//     }
// }

// proc_macro!(type_from_ty);

// proc_macro_attribute!(quantity);
// proc_macro_attribute!(unit);
// proc_macro_attribute!(exponential_scale);
// proc_macro_attribute!(with_exponential_scale);
// proc_macro_attribute!(from_unit);
// proc_macro_attribute!(into_quantity);
// proc_macro!(unit);
