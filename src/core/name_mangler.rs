use fasthash::murmur2;
use lazy_static::lazy_static;
use std::char;
use std::sync::{Arc, Mutex};

lazy_static! {
  static ref MANGLED_NAMES: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn mangle(source: &str) -> String {
  let mut names = MANGLED_NAMES.lock().unwrap();

  let mut name = String::new();
  let mut unique_identifier = 0;
  loop {
    name.push_str("rs-");
    // add a unique identifier to make more different, but many times the unique identifier ignored
    let mut hashed = murmur2::hash32(source) * 36 + unique_identifier;

    // reverse order representation of hash, but don't care
    // hash exists to make it more unique, so the order is not important
    while hashed > 0 {
      name.push(char::from_digit(hashed % 36, 36).expect("guaranteed by mod"));
      hashed /= 36;
    }

    if !names.contains(&name) {
      break;
    }

    unique_identifier += 1;
  }

  names.push(name.clone());

  name
}
