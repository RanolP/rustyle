use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec!["auto"])
    }
    fn name(&self) -> &str {
        "margin-bottom"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()
            .or(Condition::length_unit())
            .or(Condition::percentage_unit())]
    }
}
