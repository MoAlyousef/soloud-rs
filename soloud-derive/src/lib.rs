#![recursion_limit = "256"]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod audio;
mod load;
mod filter;

use crate::audio::impl_audio_trait;
use crate::load::impl_load_trait;
use crate::filter::impl_filter_trait;
use crate::filter::impl_filter_type_trait;

use proc_macro::TokenStream;

#[proc_macro_derive(AudioExt)]
pub fn audio_source_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_audio_trait(&ast)
}

#[proc_macro_derive(LoadExt)]
pub fn load_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_load_trait(&ast)
}

#[proc_macro_derive(FilterExt)]
pub fn filter_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_filter_trait(&ast)
}

#[proc_macro_derive(FilterAttr)]
pub fn filter_type_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_filter_type_trait(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
