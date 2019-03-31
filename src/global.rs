use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

lazy_static! {
  pub static ref CSS_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
  pub static ref CSS_FILES_MAP: Arc<Mutex<HashMap<String, Vec<String>>>> =
    Arc::new(Mutex::new(HashMap::new()));
  pub static ref KEYWORDS: Arc<Mutex<HashMap<String, HashSet<String>>>> =
    Arc::new(Mutex::new(HashMap::new()));
  pub static ref IS_PROPERTY_REGISTERED: Arc<Mutex<bool>> =
    Arc::new(Mutex::new(false));

  pub static ref OUTPUT: String =
    std::env::var("RUSTYLE_OUTPUT").unwrap_or(String::from("./rustyle"));

  // Listed at https://stackoverflow.com/questions/5411026/list-of-css-vendor-prefixes
  pub static ref VENDOR_PREFIXES: Vec<&'static str> = vec![
    "-ms-",     // Microsoft
    "mso-",     // Microsoft Office
    "-moz-",    // Mozilla Foundation
    "-o-",      // Opera Software
    "-xv-",     // Opera Software
    "-atsc-",   // Advanced Television Standards Committee
    "-wap-",    // The WAP Forum
    "-webkit-", // Safari, Chrome (and other WebKit-based browsers)
    "-khtml-",  // Konqueror Browser
    "-konq-",   // Konqueror Browser
    "-apple-",  // Webkit supports properties using the -apple- prefixes as well
    "prince-",  // YesLogic
    "-ah-",     // AntennaHouse
    "-hp-",     // Hewlett Packard
    "-ro-",     // Real Objects
    "-rim-",    // Research In Motion
    "-tc-",     // Tall Components
  ];
}