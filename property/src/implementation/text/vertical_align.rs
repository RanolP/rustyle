use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec![
            "baseline",
            "sub",
            "super",
            "top",
            "text-top",
            "middle",
            "bottom",
            "text-bottom",
        ])
    }
    fn name(&self) -> &str {
        "vertical-align"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword().or(Condition::percentage_unit())]
    }
}
