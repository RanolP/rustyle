use crate::core::property::{register_property, util, Parameter, Property};

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

    fn verify(&self, parameters: &Vec<Parameter>) -> bool {
        util::when_size_1(parameters, |parameter| {
            util::when_keyword(parameter, |parameter| self.check_keyword(parameter))
        })
    }
}
