use crate::core::property::{register_property, util, Parameter, Property};

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

    fn verify(&self, parameters: &Vec<Parameter>) -> bool {
        util::when_size_1(parameters, |parameter| {
            util::when_keyword(parameter, |parameter| self.check_keyword(parameter))
                || util::is_length_unit(parameter)
                || util::is_percentage_unit(parameter)
        })
    }
}
