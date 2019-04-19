use crate::core::csstype::Cssifiable;

#[derive(Clone, Debug, PartialEq)]
pub enum CssKeywordType {
    Initial,
    Inherit,
    Unset,
    NotWide(String),
}

#[derive(Clone, Debug)]
pub struct CssKeyword {
    pub origin: String,
    pub keyword_type: CssKeywordType,
}

impl Cssifiable for CssKeyword {
    fn origin(&self) -> String {
        self.origin.clone()
    }

    fn cssify(&self) -> String {
        match &self.keyword_type {
            CssKeywordType::Inherit => "inherit".into(),
            CssKeywordType::Initial => "initial".into(),
            CssKeywordType::Unset => "unset".into(),
            CssKeywordType::NotWide(s) => s.clone(),
        }
    }
}

impl From<String> for CssKeyword {
    fn from(input: String) -> CssKeyword {
        let keyword_type = match input.to_lowercase().as_str() {
            "inherit" => CssKeywordType::Inherit,
            "initial" => CssKeywordType::Initial,
            "unset" => CssKeywordType::Unset,
            _ => CssKeywordType::NotWide(input.clone()),
        };

        CssKeyword {
            origin: input.clone(),
            keyword_type: keyword_type,
        }
    }
}
