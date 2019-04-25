use crate::core::csstype::Cssifiable;
use crate::core::property::{register_property, util, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        // absolute size
        self.register_keyword(vec![
            "xx-small", "x-small", "small", "medium", "large", "x-large", "xx-large",
        ]);
        // relative size
        self.register_keyword(vec!["larger", "smaller"]);
        register_property(Instance);
    }
    fn name(&self) -> &str {
        "font-size"
    }

    fn verify(&self, arg: &Box<dyn Cssifiable>) -> bool {
        if let Some(arg) = util::as_keyword(arg) {
            self.check_keyword(arg)
        } else {
            util::is_length_unit(arg) || util::is_percentage_unit(arg)
        }
    }
}
