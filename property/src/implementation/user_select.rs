use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        [
            Keyword::simple_vec(vec!["none", "auto", "text", "contain", "all"]),
            Keyword::prefixed_vec("-moz-", vec!["none", "text", "all"]),
            // "all" Doesn't work in Safari; use only "none" or "text", or else it will allow typing in the <html> container
            Keyword::prefixed_vec("-webkit-", vec!["none", "text", "all"]),
            Keyword::prefixed_vec("-ms-", vec!["none", "text", "element"]),
        ]
        .concat()
    }
    fn name(&self) -> &str {
        "user-select"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()]
    }
}
