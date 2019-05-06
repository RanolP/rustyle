use crate::{Condition, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn name(&self) -> &str {
        "color"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword().or(Condition::Color())]
    }
}
