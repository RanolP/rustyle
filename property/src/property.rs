use crate::{Condition, Keyword};
use csstype::Cssifiable;
use std::collections::{HashMap, HashSet};

pub type Parameter = Box<dyn Cssifiable>;

pub trait Property: Send + Sync {
    fn keywords(&self) -> Vec<Keyword> {
        vec![]
    }
    fn name(&self) -> &str;
    fn condition(&self) -> Vec<Condition>;
}

pub struct Registerer {
    pub properties: HashMap<String, Box<dyn Property>>,
    pub keywords: HashMap<String, HashSet<String>>,
}

impl Registerer {
    pub fn register<P>(&mut self, property: P)
    where
        P: Property,
        P: 'static,
        P: Copy,
    {
        for keyword in property.keywords() {
            if !self.keywords.contains_key(&keyword.name) {
                self.keywords.insert(keyword.name.clone(), HashSet::new());
            }

            self.keywords
                .get_mut(&keyword.name)
                .expect("Guaranteed by before insert")
                .insert(format!("{}{}", keyword.vendor_prefix, property.name()));
        }

        self.properties
            .insert(property.name().to_string(), Box::new(property));
    }
}
