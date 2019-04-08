use crate::core::csstype::Cssifiable;
use crate::global::{KEYWORDS, PROPERTIES};
use std::collections::HashSet;

pub trait Property: Send + Sync {
  fn register(&self);

  fn name(&self) -> &str;
  fn verify(&self, arg: &Cssifiable) -> bool;

  fn register_keyword_prefixed(&self, prefix: &str, keywords: Vec<&str>) {
    let mut global_keywords = KEYWORDS.lock().unwrap();

    for keyword in keywords {
      if !global_keywords.contains_key(keyword) {
        global_keywords.insert(keyword.to_string(), HashSet::new());
      }

      global_keywords
        .get_mut(&keyword.to_string())
        .expect("Guaranteed by before insert")
        .insert(format!("{}{}", prefix, self.name()));
    }
  }

  fn register_keyword(&self, keywords: Vec<&str>) {
    self.register_keyword_prefixed("", keywords);
  }
}

pub fn register_property<P>(p: P)
where
  P: Property,
  P: Sized,
  P: 'static,
{
  let mut properties = PROPERTIES.lock().unwrap();

  properties.insert(p.name().to_string(), Box::new(p));
}
