use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["inside", "outside"])
    }
    fn name(&self) -> &str {
        "list-style-position"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()]
    }
}
