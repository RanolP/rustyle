use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, util, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        self.register_keyword(vec!["auto"]);
        register_property(Instance);
    }
    fn name(&self) -> &str {
        "height"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            util::is_length_unit(arg) || util::is_percentage_unit(arg)
        }
    }
}
