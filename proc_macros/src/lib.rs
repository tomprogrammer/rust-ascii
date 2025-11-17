use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, LitChar};

#[proc_macro]
pub fn literal(input: TokenStream) -> TokenStream {
  if let Ok(str) = syn::parse(input.clone()) {
    str_literal(str)
  } else if let Ok(char) = syn::parse(input.clone()) {
    char_literal(char)
  } else {
    quote!{ compile_error!{"Expected a string or char literal."} }.into()
  }
}

fn str_literal(expr: LitStr) -> TokenStream {
  let s = expr.value();

  if !s.is_ascii() {
    return quote!{ compile_error!{"String is not valid ascii."} }.into()
  }

  let chars = s.as_bytes().iter();
  let len = s.as_bytes().len();

  // SAFETY: all elements of `chars` are valid ascii, therefore casting from `u8` to `AsciiChar` is
  // safe.
  quote!{
    {
      const __LIT: &'static [::ascii::AsciiChar; #len] = &[
        #(unsafe { ::core::mem::transmute::<u8, ::ascii::AsciiChar>(#chars) }),*
      ];
      __LIT
    }
  }.into()
}

fn char_literal(expr: LitChar) -> TokenStream {
  let s = expr.value();

  if !s.is_ascii() {
    return quote!{ compile_error!{"Char is not valid ascii."} }.into()
  }

  let char = s as u8;

  // SAFETY: `char` is an ascii byte, therefore casting from `u8` to `AsciiChar` is safe.
  quote!{
    {
      const __LIT: ::ascii::AsciiChar = {
        unsafe { ::core::mem::transmute::<u8, ::ascii::AsciiChar>(#char) }
      };
      __LIT
    }
  }.into()
}
