use crate::core::property::{register_property, util, Parameter, Property};

pub struct Instance;

impl Property for Instance {
    fn register(&self) {
        self.register_keyword(vec![
            "normal",
            "small-caps",
            "all-small-caps",
            "petite-caps",
            "all-petite-caps",
            "unicase",
            "titling-caps",
        ]);
        register_property(Instance);
    }
    fn name(&self) -> &str {
        "font-variant-caps"
    }

    fn verify(&self, parameters: &Vec<Parameter>) -> bool {
        util::when_size_1(parameters, |parameter| {
            util::when_keyword(parameter, |parameter| self.check_keyword(parameter))
        })
    }
}
