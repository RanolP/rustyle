use super::Unit;
use crate::core::csstype::Cssifiable;

#[derive(Debug)]
pub struct PercentageUnit {
    pub origin: String,
    pub val: f64,
}

impl Cssifiable for PercentageUnit {
    fn origin(&self) -> String {
        self.origin.clone()
    }

    fn cssify(&self) -> String {
        format!("{}%", self.val)
    }
}

impl Unit for PercentageUnit {}
