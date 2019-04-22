use super::Unit;
use crate::core::csstype::Cssifiable;

#[derive(Debug)]
pub struct NumberUnit {
    pub origin: String,
    pub val: f64,
}

impl Cssifiable for NumberUnit {
    fn origin(&self) -> String {
        self.origin.clone()
    }

    fn cssify(&self) -> String {
        format!("{}", self.val)
    }
}

impl Unit for NumberUnit {}
