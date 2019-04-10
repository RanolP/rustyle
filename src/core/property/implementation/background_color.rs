use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct BackgroundColor;

impl Property for BackgroundColor {
    fn register(&self) {
        self.register_keyword(vec!["transparent", "currentcolor"]);
        register_property(BackgroundColor);
    }
    fn name(&self) -> &str {
        "background-color"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            util::is_color(arg)
        }
    }
}
