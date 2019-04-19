use super::util;
use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, Property};

pub struct Display;

impl Property for Display {
    fn register(&self) {
        self.register_keyword(vec![
            "inline",
            "block",
            "list-item",
            "inline-block",
            "table",
            "inline-table",
            "table-row-group",
            "table-header-group",
            "table-footer-group",
            "table-row",
            "table-column-group",
            "table-column",
            "table-cell",
            "table-caption",
            "none",
        ]);
        register_property(Display);
    }
    fn name(&self) -> &str {
        "display"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}
