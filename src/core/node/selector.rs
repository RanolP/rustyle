use proc_macro::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorPart {
    Itself,
    Spacing,
    Class(String),
    Id(String),
}

fn stringify(part: &SelectorPart, class_name: String) -> String {
    match part {
        SelectorPart::Itself => class_name,
        SelectorPart::Spacing => " ".to_string(),
        _ => {
            Span::call_site()
                .error(format!("Not stringifiable selector part: {:?}", part))
                .emit();
            String::new()
        }
    }
}

#[derive(Debug)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

impl Selector {
    fn stringify(&self, class_name: String) -> String {
        let mut result = String::new();

        for part in &self.parts {
            result.push_str(&stringify(part, class_name.clone()));
        }

        result
    }
}

pub type SelectorGroup = Vec<Selector>;
