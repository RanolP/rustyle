use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, util, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        self.register_keyword(vec!["none", "left", "right", "both"]);
        register_property(Instance);
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
