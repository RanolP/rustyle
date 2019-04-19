use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct ListStyleType;

impl Property for ListStyleType {
    fn register(&self) {
        self.register_keyword(vec![
            "disc",
            "circle",
            "square",
            "decimal",
            "decimal-leading-zero",
            "lower-roman",
            "upper-roman",
            "lower-greek",
            "lower-latin",
            "upper-latin",
            "armenian",
            "georgian",
            "lower-alpha",
            "upper-alpha",
            "none",
        ]);
        register_property(ListStyleType);
    }
    fn name(&self) -> &str {
        "list-style-type"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}
