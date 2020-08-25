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

mod audio_source;

use crate::audio_source::impl_audio_source_trait;

use proc_macro::TokenStream;

#[proc_macro_derive(AudioSource)]
pub fn audio_source_trait_macro(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_audio_source_trait(&ast)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
