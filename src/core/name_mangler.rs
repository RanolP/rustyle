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

  let append = |name: &mut String, number: u32| {
    let mut number = number;

    if number == 0 {
      name.push('0');
      return;
    }

    // reverse order representation of hash, but don't care
    // hash exists to make it more unique, so the order is not important
    while number > 0 {
      name.push(char::from_digit(number % 36, 36).expect("guaranteed by mod"));
      number /= 36;
    }
  };

  loop {
    name.push_str("rs-");
    append(&mut name, unique_identifier);
    append(&mut name, murmur2::hash32(source));

    if !names.contains(&name) {
      break;
    }

    unique_identifier += 1;
  }

  names.push(name.clone());

  name
}
