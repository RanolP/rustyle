use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct Color;

impl Property for Color {
    fn register(&self) {
        register_property(Color);
    }
    fn name(&self) -> &str {
        "color"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            util::is_color(arg)
        }
    }
}
