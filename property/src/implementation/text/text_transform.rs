use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec![
            "capitalize",
            "uppercase",
            "lowercase",
            "none",
        ])
    }
    fn name(&self) -> &str {
        "text-transform"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()]
    }
}
