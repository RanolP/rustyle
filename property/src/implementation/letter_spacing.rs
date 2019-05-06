use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["normal"])
    }
    fn name(&self) -> &str {
        "letter-spacing"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword().or(Condition::LengthUnit())]
    }
}
