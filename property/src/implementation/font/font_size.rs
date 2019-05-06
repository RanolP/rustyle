use crate::{Condition, Keyword, Property};

#[derive(Clone, Copy)]
pub struct Instance;

impl Property for Instance {
    fn keywords(&self) -> Vec<Keyword> {
        [
            // absolute size
            Keyword::SimpleVec(vec![
                "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large",
            ]),
            // relative size
            Keyword::SimpleVec(vec!["larger", "smaller"]),
        ]
        .concat()
    }
    fn name(&self) -> &str {
        "font-size"
    }

    fn condition(&self) -> Vec<Condition> {
        vec![Condition::Keyword()
            .or(Condition::LengthUnit())
            .or(Condition::PercentageUnit())]
    }
}
