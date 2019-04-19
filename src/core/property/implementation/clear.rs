use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct Clear;

impl Property for Clear {
    fn register(&self) {
        self.register_keyword(vec!["none", "left", "right", "both"]);
        register_property(Clear);
    }
    fn name(&self) -> &str {
        "clear"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}
