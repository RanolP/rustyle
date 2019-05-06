use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        [
            // absolute size
            Keyword::simple_vec(vec![
                "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large",
            ]),
            // relative size
            Keyword::simple_vec(vec!["larger", "smaller"]),
        ]
        .concat()
    }
    fn name(&self) -> &str {
        "font-size"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()
            .or(Condition::length_unit())
            .or(Condition::percentage_unit())]
    }
}
