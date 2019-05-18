use csstype::CssUnitGroup;

#[derive(Clone, PartialEq)]
pub enum ConditionType {
    Keyword,
    Color,
    Unit(Vec<CssUnitGroup>),
    ValueAllocatedUnit(CssUnitGroup, f32),
}

impl ConditionType {
    fn into_condition(self) -> Condition {
        Condition {
            types_variant: vec![self],
        }
    }
}

pub struct Condition {
    pub types_variant: Vec<ConditionType>,
}

impl Condition {
    pub fn keyword() -> Condition {
        ConditionType::Keyword.into_condition()
    }
    pub fn color() -> Condition {
        ConditionType::Color.into_condition()
    }
    pub fn length_unit() -> Condition {
        ConditionType::Unit(vec![
            CssUnitGroup::Integer,
            CssUnitGroup::Number,
            CssUnitGroup::AbsoluteLength,
            CssUnitGroup::FontRelativeLength,
            CssUnitGroup::ViewportRelativeLength,
        ])
        .into_condition()
    }
    pub fn percentage_unit() -> Condition {
        ConditionType::Unit(vec![CssUnitGroup::Percentage]).into_condition()
    }
    pub fn integer_exact(number: i32) -> Condition {
        ConditionType::ValueAllocatedUnit(CssUnitGroup::Integer, number as f32).into_condition()
    }
    pub fn number_exact(number: f32) -> Condition {
        ConditionType::ValueAllocatedUnit(CssUnitGroup::Number, number).into_condition()
    }
    pub fn or(self, cond: Condition) -> Self {
        Condition {
            types_variant: [self.types_variant, cond.types_variant].concat(),
        }
    }
}
