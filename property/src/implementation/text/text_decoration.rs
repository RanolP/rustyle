use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec![
            "none",
            "underline",
            "overline",
            "line-through",
            "blink",
        ])
    }
    fn name(&self) -> &str {
        "text-decoration"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()]
    }
}
