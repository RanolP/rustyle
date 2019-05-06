use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["left", "right", "none"])
    }
    fn name(&self) -> &str {
        "float"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()]
    }
}
