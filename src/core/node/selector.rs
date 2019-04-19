use proc_macro::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    Itself,
    Spacing,
    Class(String),
    Id(String),
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
    PseudoElement(String),
}

fn stringify(part: &SelectorPart, class_name: String) -> String {
    #[allow(unreachable_patterns)]
    match part {
        SelectorPart::Itself => format!(".{}", class_name),
        SelectorPart::Spacing => " ".to_string(),
        SelectorPart::Class(s) => format!(".{}", s),
        SelectorPart::Id(s) => format!("#{}", s),
        SelectorPart::Element { namespace, name } => format!(
            "{}{}",
            namespace
                .clone()
                .map_or("".to_string(), |namespace| format!("{}|", namespace)),
            name
        ),
        SelectorPart::Universal { namespace } => format!(
            "{}*",
            namespace
                .clone()
                .map_or("".to_string(), |namespace| format!("{}|", namespace)),
        ),
        SelectorPart::PseudoElement(name) => format!("::{}", name),
        SelectorPart::PseudoClass { name, parameter } => format!(
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
