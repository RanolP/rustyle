use crate::core::node::{DeclarationNode, MetadataNode, MetadataType, RulesetNode};
use crate::core::parse::{parse_declaration, parse_metadata};
use proc_macro::{Delimiter, TokenTree};
use std::iter::Peekable;

struct DeclarationShouldWarn {
  vendor_prefix: bool
}

pub fn parse_ruleset<I: 'static>(tokens: &mut Peekable<I>) -> Option<Box<RulesetNode>>
where
  I: Iterator<Item = TokenTree>,
{
  let mut declarations = Vec::<DeclarationNode>::new();
  let mut ruleset_metadatas = Vec::<MetadataNode>::new();
  let mut rule_metadatas = Vec::<MetadataNode>::new();
  let mut first = None;
  let mut last = None;

  let mut parse_declaration = |rule_metadatas: &mut Vec<MetadataNode>, tokens: &mut Peekable<I>| {
    let parsed = parse_declaration(tokens);

    if let Some(node) = parsed {
      if first.is_none() {
        first = Some(node.range);
      }
      last = Some(node.range);

      let mut should_warn = DeclarationShouldWarn {
        vendor_prefix: true,
      };

      for metadata in rule_metadatas.iter_mut() {
        // todo: separate hard-coded metadata strategies
        if metadata.method_name == "no_warn" {
          match metadata.parameters.len() {
            0 => {
              metadata.range.error("one parameter expected but no parameter received").emit();
              continue;
            },
            1 => {},
            _ => {
              metadata.range.warning("2 or more parameters received").emit();
            }
          }

          match metadata.parameters[0].as_str() {
            "vendor_prefix" => {
              should_warn.vendor_prefix = false;
            }
            param @ _ => {
              metadata.range.error(format!("Unexpected parameter {}", param)).emit();
            }
          }
        }
      }

      if node.prefix.len() > 0 && should_warn.vendor_prefix {
        node.range.warning("Consider removing the vendor prefix").emit();
      }

      declarations.push(node);
    }
  };

  loop {
    match tokens.peek().cloned() {
      Some(TokenTree::Punct(ref token)) if token.as_char() == '#' => {
        let sharp = tokens.next().expect("Guaranteed by match");
        // todo: unwrap_or(parse_selector())
        
        let parsed = parse_metadata(sharp, tokens);
        
        match parsed {
          Some(node @ MetadataNode {
            metadata_type: MetadataType::Ruleset,
            ..
          }) => {
            ruleset_metadatas.push(node);
          },
          Some(node @ MetadataNode {
            metadata_type: MetadataType::Rule,
            ..
          }) => {
            rule_metadatas.push(node);
          }
          _ => {
            panic!("Never happen")
          }
        }

        continue;
      }
      Some(TokenTree::Punct(ref token))
        // class selector
        if token.as_char() == '.'
        // itself selector
        || token.as_char() == '&'
        // universal selector
        || token.as_char() == '*'
        // state selector
        || token.as_char() == ':'
        // adjacent sibling selector
        || token.as_char() == '+'
        // general sibling selector
        || token.as_char() == '~'
        // child selector
        || token.as_char() == '>' =>
      {
        // todo: parse_selector()
        break;
      }
      Some(TokenTree::Group(ref token)) if token.delimiter() == Delimiter::Bracket => {
        // todo: parse_selector()
        break;
      }
      Some(TokenTree::Ident(_))  => {
        parse_declaration(&mut rule_metadatas, tokens);
      }
      Some(TokenTree::Punct(ref token)) if token.as_char() == '-' => {
        parse_declaration(&mut rule_metadatas, tokens);
      }
      Some(TokenTree::Punct(ref token)) if token.as_char() == ';' => {
        parse_declaration(&mut rule_metadatas, tokens);
      }
      None => {
        break;
      }
      Some(token) => {
        token.span().error(format!("Unacceptable token {:?}", token.to_string())).emit();
        return None;
      }
    }
  }

  if declarations.is_empty() {
    None
  } else {
    Some(Box::new(RulesetNode {
      range: if let Some(first) = first {
        Some(first.join(last.unwrap_or(first)).expect("In the same file"))
      } else {
        None
      },
      declarations: declarations,
      metadatas: ruleset_metadatas,
    }))
  }
}
