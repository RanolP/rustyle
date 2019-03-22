use proc_macro::TokenStream;
use quote::quote;
use std::iter::FromIterator;

#[doc(hidden)]
pub fn css_use_impl(item: TokenStream) -> TokenStream {
  // todo: value inlining
  // println!("CSS USE\n{:?}\n", item);

  // the variable maybe used by rustyle
  // so should ignore the unused warning
  let preamble_tokens = TokenStream::from(quote! {
    #[allow(unused)]
  });

  TokenStream::from_iter(preamble_tokens.into_iter().chain(item).into_iter())
}
