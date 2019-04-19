use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct ListStylePosition;

impl Property for ListStylePosition {
    fn register(&self) {
        self.register_keyword(vec!["inside", "outside"]);
        register_property(ListStylePosition);
    }
    fn name(&self) -> &str {
        "list-style-position"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}
