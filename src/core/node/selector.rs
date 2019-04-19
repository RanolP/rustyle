use proc_macro::Span;

#[derive(Debug, Clone)]
pub enum SelectorPart {
    Itself {
        span: Span,
    },
    Spacing,
    Class {
        span: Span,
        name: String,
    },
    Id {
        span: Span,
        name: String,
    },
    Element {
        span: Span,
        namespace: Option<String>,
        name: String,
    },
    Universal {
        span: Span,
        namespace: Option<String>,
    },
    PseudoClass {
        span: Span,
        name: String,
        // todo: parameter validation required?
        parameter: Option<String>,
    },
    PseudoElement {
        span: Span,
        name: String,
    },
}

impl PartialEq for SelectorPart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SelectorPart::Itself { .. }, SelectorPart::Itself { .. }) => true,
            (SelectorPart::Spacing, SelectorPart::Spacing) => true,
            (
                SelectorPart::Class {
                    name: left_name, ..
                },
                SelectorPart::Class {
                    name: right_name, ..
                },
            ) => left_name == right_name,
            (
                SelectorPart::Id {
                    name: left_name, ..
                },
                SelectorPart::Id {
                    name: right_name, ..
                },
            ) => left_name == right_name,
            (
                SelectorPart::Element {
                    namespace: left_namespace,
                    name: left_name,
                    ..
                },
                SelectorPart::Element {
                    namespace: right_namespace,
                    name: right_name,
                    ..
                },
            ) => left_namespace == right_namespace && left_name == right_name,
            (
                SelectorPart::Universal {
                    namespace: left_namespace,
                    ..
                },
                SelectorPart::Universal {
                    namespace: right_namespace,
                    ..
                },
            ) => left_namespace == right_namespace,
            (
                SelectorPart::PseudoClass {
                    name: left_name,
                    parameter: left_parameter,
                    ..
                },
                SelectorPart::PseudoClass {
                    name: right_name,
                    parameter: right_parameter,
                    ..
                },
            ) => left_name == right_name && left_parameter == right_parameter,
            (
                SelectorPart::PseudoElement {
                    name: left_name, ..
                },
                SelectorPart::PseudoElement {
                    name: right_name, ..
                },
            ) => left_name == right_name,
            _ => false,
        }
    }
}

impl SelectorPart {
    pub fn span(&self) -> Option<Span> {
        match self {
            SelectorPart::Itself { span, .. }
            | SelectorPart::Class { span, .. }
            | SelectorPart::Id { span, .. }
            | SelectorPart::Element { span, .. }
            | SelectorPart::Universal { span, .. }
            | SelectorPart::PseudoClass { span, .. }
            | SelectorPart::PseudoElement { span, .. } => Some(*span),
            _ => None,
        }
    }
}

fn stringify(part: &SelectorPart, class_name: String) -> String {
    #[allow(unreachable_patterns)]
    match part {
        SelectorPart::Itself { .. } => format!(".{}", class_name),
        SelectorPart::Spacing => " ".to_string(),
        SelectorPart::Class { name, .. } => format!(".{}", name),
        SelectorPart::Id { name, .. } => format!("#{}", name),
        SelectorPart::Element {
            namespace, name, ..
        } => format!(
            "{}{}",
            namespace
                .clone()
                .map_or("".to_string(), |namespace| format!("{}|", namespace)),
            name
        ),
        SelectorPart::Universal { namespace, .. } => format!(
            "{}*",
            namespace
                .clone()
                .map_or("".to_string(), |namespace| format!("{}|", namespace)),
        ),
        SelectorPart::PseudoElement { name, .. } => format!("::{}", name),
        SelectorPart::PseudoClass {
            name, parameter, ..
        } => format!(
            ":{}{}",
            name,
            if let Some(parameter) = parameter {
                format!("({})", parameter)
            } else {
                "".to_string()
            }
        ),

        _ => {
            Span::call_site()
                .error(format!("Not stringifiable selector part: {:?}", part))
                .emit();
            String::new()
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl Selector {
    pub fn stringify(&self, class_name: String) -> String {
        let mut result = String::new();

        for part in &self.parts {
            result.push_str(&stringify(part, class_name.clone()));
        }

        result
    }
}

pub type SelectorGroup = Vec<Selector>;
