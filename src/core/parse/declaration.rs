use crate::core::node::DeclarationNode;
use crate::core::parse::parse_expression;
use proc_macro::{Span, TokenTree};

pub fn parse_declaration<I: 'static>(tokens: &mut I) -> Option<DeclarationNode>
where
  I: Iterator<Item = TokenTree>,
{
  let mut key = Vec::<TokenTree>::new();
  let mut ignore_tails = false;

  while let Some(token) = tokens.next() {
    if ignore_tails {
      if let TokenTree::Punct(ref punct) = token {
        if punct.as_char() == ';' {
          break;
        }
      }
      continue;
    }

    match token {
      TokenTree::Punct(ref punct) if punct.as_char() == ';' => {
        if key.is_empty() {
          return None;
        }
      }
      TokenTree::Punct(ref punct) if punct.as_char() == ':' => {
        let mut key_str = String::new();
        let mut last_span: Option<Span> = None;
        let mut spaced = false;

        let report = |span: Span| span.help("Consider removing space").emit();

        let mut iter = key.iter();
        while let Some(token) = iter.next() {
          if let Some(end_span) = last_span {
            let start_span = token.span();
            if end_span.end() != start_span.start() {
              spaced = true;
              last_span = end_span.join(start_span);
            } else {
              if spaced {
                spaced = false;
                report(last_span.unwrap());
              }

              last_span = Some(token.span());
            }
          } else {
            last_span = Some(token.span());
          }

          key_str.push_str(&token.to_string());
        }

        if spaced {
          if let Some(span) = last_span {
            report(span);
          }
        }

        let expr = parse_expression(tokens);

        if let Some(expr) = expr {
          let result = DeclarationNode {
            name: key_str,
            value: Box::new(expr),
          };
          return Some(result);
        } else {
          ignore_tails = true;
        }
      }
      _ => {
        key.push(token);
      }
    }
  }

  None
}
