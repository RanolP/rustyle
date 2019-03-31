use crate::core::csstype::Cssifiable;
use crate::global::KEYWORDS;
use std::collections::HashSet;

pub trait Property {
  fn register();
  fn name() -> &'static str;
  fn verify(arg: &Cssifiable) -> bool;

  fn register_keyword_prefixed(prefix: &'static str, keywords: Vec<&'static str>) {
    let mut global_keywords = KEYWORDS.lock().unwrap();

    for keyword in keywords {
      if !global_keywords.contains_key(keyword) {
        global_keywords.insert(keyword.to_string(), HashSet::new());
      }

      global_keywords
        .get_mut(&keyword.to_string())
        .expect("Guaranteed by before insert")
        .insert(format!("{}{}", prefix, Self::name()));
    }
  }

  fn register_keyword(keywords: Vec<&'static str>) {
    Self::register_keyword_prefixed("", keywords)
  }
}
