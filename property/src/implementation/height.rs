use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        Keyword::SimpleVec(vec!["auto"])
    }
    fn name(&self) -> &str {
        "height"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()
            .or(Condition::LengthUnit())
            .or(Condition::PercentageUnit())]
    }
}
