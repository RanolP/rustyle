use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["auto", "normal", "none"])
    }
    fn name(&self) -> &str {
        "font-kerning"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()]
    }
}
