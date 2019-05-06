use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["normal", "sub", "super"])
    }
    fn name(&self) -> &str {
        "font-variant-position"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()]
    }
}
