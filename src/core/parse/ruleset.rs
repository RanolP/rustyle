use crate::core::node::{DeclarationNode, MetadataNode, MetadataType, RulesetNode};
use crate::core::parse::{parse_declaration, parse_metadata};
use crate::core::metadata::RuleMetadataProcessor;
use crate::global::RULE_METADATA_PROCESSORS;
use proc_macro::{Delimiter, TokenTree, Span};
use std::iter::Peekable;
use std::collections::HashMap;

pub fn parse_ruleset<I: 'static>(tokens: &mut Peekable<I>) -> Option<Box<RulesetNode>>
where
  I: Iterator<Item = TokenTree>,
{
  let rule_metadata_processors = RULE_METADATA_PROCESSORS.lock().unwrap();

  let mut declarations = Vec::<DeclarationNode>::new();
  let mut ruleset_metadatas = Vec::<MetadataNode>::new();
  let mut rule_metadatas = Vec::<MetadataNode>::new();
  let mut first = None;
  let mut last = None;

  let mut parse_declaration = |rule_metadatas: &mut Vec<MetadataNode>, tokens: &mut Peekable<I>| {
    let parsed = parse_declaration(rule_metadatas.to_vec(), tokens);

    if let Some(node) = parsed {
      if first.is_none() {
        first = Some(node.range);
      }
      last = Some(node.range);

      let mut processors = HashMap::<String, (&Box<RuleMetadataProcessor>, Vec<MetadataNode>)>::new();

      for processor in rule_metadata_processors.values() {
        processors.insert(processor.name().to_string(), (processor, Vec::new()));
      }

      for metadata in node.metadatas.clone() {
        if !processors.contains_key(&metadata.method_name) {
            metadata.range.error("Unknown metadata").emit();
            continue;
        }
        
        processors.get_mut(&metadata.method_name.clone()).expect("Guaranteed by before if").1.push(metadata);
      }

      for (processor, metadatas) in processors.values() {
        (*processor).process(&node, metadatas.to_vec());
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
