use runtime::CompileContext;
use metadata::RuleMetadataProcessor;
use node::{MetadataNode, DeclarationNode};
use runtime::global::{PROPERTIES, RULE_METADATA_PROCESSORS, KEYWORDS};
use std::collections::HashMap;
use property::ConditionType;
use crate::CodeGenerator;
use csstype::{RgbColor, HslColor, CssUnit, Cssifiable, CssKeyword, CssKeywordType};

impl CodeGenerator for DeclarationNode {
    fn generate_code(&self, _: &str, _: &mut CompileContext) -> String {
        let rule_metadata_processors = RULE_METADATA_PROCESSORS.lock().unwrap();

        let mut processors =
            HashMap::<String, (&Box<dyn RuleMetadataProcessor>, Vec<MetadataNode>)>::new();

        for processor in rule_metadata_processors.values() {
            processors.insert(processor.name().to_string(), (processor, Vec::new()));
        }

        for metadata in self.metadatas.clone() {
            if !processors.contains_key(&metadata.method_name) {
                metadata.range.error("Unknown metadata").emit();
                continue;
            }

            processors
                .get_mut(&metadata.method_name.clone())
                .expect("Guaranteed by before if")
                .1
                .push(metadata);
        }

        for (processor, metadatas) in processors.values() {
            (*processor).process(&self, metadatas.to_vec());
        }

        let properties = PROPERTIES.lock().unwrap();
        let keywords = KEYWORDS.lock().unwrap();

        match properties.get(&self.name) {
            Some(property) => {
                let condition = property.condition();
                let join_value = |value: &Vec<Box<dyn Cssifiable>>| {
                            value
                                .iter()
                                .map(|value| value.origin())
                                .collect::<Vec<String>>()
                                .join(" ")
                };
                if self.value.len() != condition.len() {
                    self.range
                        .error(format!(
                            "Expected parameter count: {}, Received parameter count: {}",
                            condition.len(),
                            self.value.len()
                        ))
                        .emit();
                } else {
                    for (condition, value) in condition.into_iter().zip((&self.value).into_iter()) {
                        let any = value.as_any();
                        if (any.is::<HslColor>() || any.is::<RgbColor>()) && condition.types_variant.contains(&ConditionType::Color) {
                            continue;
                        }
                        if let Some(keyword) = any.downcast_ref::<CssKeyword>() {
                            if condition.types_variant.contains(&ConditionType::Keyword) {
                            match &keyword.keyword_type {
                                CssKeywordType::NotWide(s) => {
                                    if keywords.get(s).map(|set| set.contains(&format!("{}{}", self.prefix, self.name))).unwrap_or(false) {
                                        continue;
                                    } else {
                                        self.range.error(format!("Unacceptable keyword `{}` on {}{}`", s, self.prefix, self.name)).emit();
                                        break;
                                    }
                                }
                                ,_ => {continue;}
                            }
                            }
                        }
                        if let Some(unit) = any.downcast_ref::<CssUnit>() {
                            if condition.types_variant.iter().any(|t| {
                                if let ConditionType::Unit(groups) = t {
                                    groups.contains(&unit.group)
                                } else {
                                    false
                                }
                            }) {
                                continue;
                            }
                        }

                    self.range
                        .error(format!(
                            "Unacceptable data `{}` on `{}{}`",
                            join_value(&self.value),
                            self.prefix,
                            self.name
                        ))
                        .emit();
                        break;
                    }
                }
            }
            None => {
                self.range
                    .warning(format!("Unknown property {}", self.name))
                    .emit();
            }
        }

        let value = &*self.value;
        return format!(
            "{prefix}{key}: {value};",
            prefix = self.prefix,
            key = self.name,
            value = value
                .iter()
                .map(|value| value.optimized_cssify())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
