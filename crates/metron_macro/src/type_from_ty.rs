use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, UseTree};
use crate::util::ident_of;

pub fn type_from_ty(item: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(item as UseTree);
    let name = {
        fn get_tail(tree: UseTree) -> UseTree{
            match tree {
                UseTree::Path(path) => get_tail(*path.tree),
                UseTree::Name(_) => tree,
                _ => panic!()
            }
        }
        if let UseTree::Name(name) = get_tail(tree){
            name.ident
        }else{
            panic!()
        }
    };
    let expanded = quote! {
        #name
    };
    expanded.into()
}
