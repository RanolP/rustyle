use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords() -> Vec<Keyword> {
        Keyword::simple_vec(vec!["normal"])
    }
    fn name(&self) -> &str {
        "line-height"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::length_unit().or(Condition::percentage_unit())]
    }
}
