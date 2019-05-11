use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec!["normal", "bold", "bolder", "lighter"])
    }
    fn name(&self) -> &str {
        "font-weight"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()
            .or(Condition::integer_exact(100))
            .or(Condition::integer_exact(200))
            .or(Condition::integer_exact(300))
            .or(Condition::integer_exact(400))
            .or(Condition::integer_exact(500))
            .or(Condition::integer_exact(600))
            .or(Condition::integer_exact(700))
            .or(Condition::integer_exact(800))
            .or(Condition::integer_exact(900))]
    }
}
