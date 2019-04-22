use proc_macro::TokenStream;
use quote::quote;
use std::iter::FromIterator;

pub fn css_use(item: TokenStream) -> TokenStream {
    // todo: value inlining
    // println!("CSS USE\n{:?}\n", item);

    // ? The variable maybe used by rustyle, so should ignore the unused warning
    let preamble_tokens = TokenStream::from(quote! {
      #[allow(unused)]
    });

    TokenStream::from_iter(preamble_tokens.into_iter().chain(item).into_iter())
}
