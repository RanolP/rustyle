use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn name(&self) -> &str {
        "padding-right"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::length_unit()
            .or(Condition::percentage_unit())]
    }
}
