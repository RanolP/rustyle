use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, util, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        self.register_keyword(vec!["none", "auto", "text", "contain", "all"]);

        self.register_keyword_prefixed("-moz-", vec!["none", "text", "all"]);
        // "all" Doesn't work in Safari; use only "none" or "text", or else it will allow typing in the <html> container
        self.register_keyword_prefixed("-webkit-", vec!["none", "text", "all"]);
        self.register_keyword_prefixed("-ms-", vec!["none", "text", "element"]);

        register_property(Instance);
    }
    fn name(&self) -> &str {
        "user-select"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            false
        }
    }
}
