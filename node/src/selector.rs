use proc_macro::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPartType {
    Itself,
    Spacing,
    Class {
        name: String,
    },
    Id {
        name: String,
    },
    Element {
        namespace: Option<String>,
        name: String,
    },
    Universal {
        namespace: Option<String>,
    },
    PseudoClass {
        name: String,
        // todo: parameter validation required?
        parameter: Option<String>,
    },
    PseudoElement {
        name: String,
    },
    Child {
        selector: Selector,
    },
    NextSibling {
        selector: Selector,
    },
    SubsequentSibling {
        selector: Selector,
    },
}

pub type SelectorPart = (SelectorPartType, Option<Span>);

fn stringify(part: &SelectorPartType, class_name: String) -> String {
    #[allow(unreachable_patterns)]
    match part {
        SelectorPartType::Itself => format!(".{}", class_name),
        SelectorPartType::Spacing => " ".to_string(),
        SelectorPartType::Class { name } => format!(".{}", name),
        SelectorPartType::Id { name } => format!("#{}", name),
        SelectorPartType::Element { namespace, name } => format!(
            "{}{}",
            namespace
                .clone()
                .map_or("".to_string(), |namespace| format!("{}|", namespace)),
            name
        ),
        SelectorPartType::Universal { namespace } => format!(
            "{}*",
            namespace
                .clone()
                .map_or("".to_string(), |namespace| format!("{}|", namespace)),
        ),
        SelectorPartType::PseudoElement { name } => format!("::{}", name),
        SelectorPartType::PseudoClass { name, parameter } => format!(
            ":{}{}",
            name,
            if let Some(parameter) = parameter {
                format!("({})", parameter)
            } else {
                "".to_string()
            }
        ),
        SelectorPartType::Child { selector } => format!(">{}", selector.stringify(class_name)),

        _ => {
            Span::call_site()
                .error(format!("Not stringifiable selector part: {:?}", part))
                .emit();
            String::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl PartialEq for Selector {
    fn eq(&self, other: &Self) -> bool {
        for (left, right) in self.parts.iter().zip(other.parts.iter()) {
            if left.0 != right.0 {
                return false;
            }
        }
        true
    }
}

impl Selector {
    pub fn stringify(&self, class_name: String) -> String {
        let mut result = String::new();

        for part in &self.parts {
            result.push_str(&stringify(&part.0, class_name.clone()));
        }

        result
    }

    pub fn span(&self) -> Option<Span> {
        if let (Some(first), Some(last)) = (self.parts.first(), self.parts.last()) {
            first.1.map_or(last.1, |first| {
                last.1
                    .map(|last| first.join(last).expect("In the same file"))
                    .or(Some(first))
            })
        } else {
            None
        }
    }
}

pub type SelectorGroup = Vec<Selector>;
