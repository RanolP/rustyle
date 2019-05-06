use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec!["transparent", "currentcolor"])
    }
    fn name(&self) -> &str {
        "background-color"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword().or(Condition::color())]
    }
}
