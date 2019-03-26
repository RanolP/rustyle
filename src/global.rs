use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
  pub static ref CSS_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
  pub static ref CSS_FILES_MAP: Arc<Mutex<HashMap<String, Vec<String>>>> =
    Arc::new(Mutex::new(HashMap::new()));
  pub static ref OUTPUT: String =
    std::env::var("RUSTYLE_OUTPUT").unwrap_or(String::from("./rustyle"));
}
