use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        [
            Keyword::SimpleVec(vec!["none", "auto", "text", "contain", "all"]),
            Keyword::PrefixedVec("-moz-", vec!["none", "text", "all"]),
            // "all" Doesn't work in Safari; use only "none" or "text", or else it will allow typing in the <html> container
            Keyword::PrefixedVec("-webkit-", vec!["none", "text", "all"]),
            Keyword::PrefixedVec("-ms-", vec!["none", "text", "element"]),
        ]
        .concat()
    }
    fn name(&self) -> &str {
        "user-select"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()]
    }
}
