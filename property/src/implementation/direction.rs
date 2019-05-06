use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["ltr", "rtl"])
    }
    fn name(&self) -> &str {
        "direction"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()]
    }
}
