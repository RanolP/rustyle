use crate::core::csstype::{ColorParseError, Cssifiable, RgbColor};
use proc_macro::{Span, TokenTree};

pub fn parse_color<I>(tokens: &mut I) -> Option<impl Cssifiable>
where
  I: Iterator<Item = TokenTree>,
{
  if let Some(first_token) = tokens.next() {
    match first_token {
      TokenTree::Punct(ref punct) if punct.as_char() == '#' => parse_color_hex(first_token, tokens),
      _ => None,
    }
  } else {
    None
  }
}

pub fn parse_color_hex<I>(sharp: TokenTree, tokens: &mut I) -> Option<RgbColor>
where
  I: Iterator<Item = TokenTree>,
{
  let invalid_hex = |span: Span| span.error("Invalid hex color").emit();
  if let Some(token) = tokens.next() {
    let parsed_color = format!("#{}", token.to_string()).parse::<RgbColor>();
    match parsed_color {
      Ok(color) => Some(color),
      Err(cause) => match cause {
        ColorParseError::StringEmpty | ColorParseError::NotAHexColor => {
          panic!("guaranteed by if let")
        }
        ColorParseError::InvalidHexColor => {
          invalid_hex(
            sharp
              .span()
              .join(token.span())
              .expect("Two tokens are in the same file, guaranteed by the caller"),
          );
          None
        }
      },
    }
  } else {
    invalid_hex(sharp.span());
    None
  }
}
