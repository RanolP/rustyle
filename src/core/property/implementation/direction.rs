use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct Direction;

impl Property for Direction {
    fn register(&self) {
        self.register_keyword(vec!["ltr", "rtl"]);
        register_property(Direction);
    }
    fn name(&self) -> &str {
        "direction"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}
