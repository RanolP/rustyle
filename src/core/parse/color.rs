use crate::core::csstype::{ColorParseError, Cssifiable, RgbColor};
use proc_macro::{Span, TokenTree};

pub fn parse_color_hex<I>(
  sharp: TokenTree,
  tokens: &mut I,
) -> (Option<Box<dyn Cssifiable>>, Option<Span>)
where
  I: Iterator<Item = TokenTree>,
{
  let invalid_hex = |span: Span| span.error("Invalid hex color").emit();
  if let Some(token) = tokens.next() {
    let parsed_color = RgbColor::parse_hex(&format!("#{}", token.to_string()));
    match parsed_color {
      Ok(color) => (Some(Box::new(color)), Some(token.span())),
      Err(cause) => match cause {
        ColorParseError::StringEmpty | ColorParseError::NotAHexColor => {
          panic!("guaranteed by if let")
        }
        ColorParseError::InvalidHexColor => {
          invalid_hex(sharp.span().join(token.span()).expect("In the same file"));
          (
            None,
            Some(sharp.span().join(token.span()).expect("In the same file")),
          )
        }
      },
    }
  } else {
    invalid_hex(sharp.span());
    (None, Some(sharp.span()))
  }
}
