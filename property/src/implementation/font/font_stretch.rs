use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::simple_vec(vec![
            "normal",
            "ultra-condensed",
            "extra-condensed",
            "condensed",
            "semi-condensed",
            "semi-expanded",
            "expanded",
            "extra-expanded",
            "ultra-expanded",
        ])
    }
    fn name(&self) -> &str {
        "font-stretch"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::keyword()]
    }
}
