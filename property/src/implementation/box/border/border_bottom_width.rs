use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec!["thin", "medium", "thick"])
    }
    fn name(&self) -> &str {
        "border-bottom-width"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()
            .or(Condition::length_unit())]
    }
}
