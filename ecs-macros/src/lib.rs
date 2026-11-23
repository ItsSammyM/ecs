
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Component)]
pub fn component_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    quote! {
        impl ecs::prelude::Component for #name {}
    }.into()
}

#[proc_macro_derive(ComponentInCrate)]
pub fn component_derive_in_crate(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    quote! {
        impl crate::prelude::Component for #name {}
    }.into()
}