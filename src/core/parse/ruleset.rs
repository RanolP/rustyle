use crate::core::node::{DeclarationNode, MetadataNode, MetadataType, RulesetNode};
use crate::core::parse::{parse_declaration, parse_metadata};
use proc_macro::{Delimiter, TokenTree};
use std::iter::Peekable;

pub fn parse_ruleset<I: 'static>(tokens: &mut Peekable<I>, is_root: bool) -> Option<RulesetNode>
where
    I: Iterator<Item = TokenTree>,
{
    let mut declarations = Vec::<DeclarationNode>::new();
    let mut root_metadatas = Vec::<MetadataNode>::new();
    let mut rule_metadatas = Vec::<MetadataNode>::new();
    let mut first = None;
    let mut last = None;

    let mut parse_declaration = |rule_metadatas: &mut Vec<MetadataNode>,
                                 declarations: &mut Vec<DeclarationNode>,
                                 tokens: &mut Peekable<I>| {
        let parsed = parse_declaration(rule_metadatas.to_vec(), tokens);

        if let Some(node) = parsed {
            if first.is_none() {
                first = Some(node.range);
            }
            last = Some(node.range);

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
            metadata_type: MetadataType::Root,
            ..
          }) => {
            if !rule_metadatas.is_empty() || !declarations.is_empty() {
              node.range.warning("Put root metadata on the first of ruleset").emit();
            }
            if !is_root {
              node.range.error("Put root metadata on the root of ruleset").emit();
              continue;
            }
            root_metadatas.push(node);
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
        parse_declaration(&mut rule_metadatas, &mut declarations, tokens);
      }
      Some(TokenTree::Punct(ref token)) if token.as_char() == '-' => {
        parse_declaration(&mut rule_metadatas,&mut declarations, tokens);
      }
      Some(TokenTree::Punct(ref token)) if token.as_char() == ';' => {
        parse_declaration(&mut rule_metadatas, &mut declarations,tokens);
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
        Some(RulesetNode {
            range: if let Some(first) = first {
                Some(first.join(last.unwrap_or(first)).expect("In the same file"))
            } else {
                None
            },
            declarations: declarations,
            metadatas: root_metadatas,
            is_root: is_root,
        })
    }
}
