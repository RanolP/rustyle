#![feature(proc_macro_hygiene)]
use rustyle::{css_use, rustyle};

fn main() {
  #[css_use]
  let red = "gray";

  let test = rustyle! {
    background-color: #00cccc;
    #[allow(vendor_prefix)]
    -moz-user-select: none;
    background-color: ${red};
  };

  println!("{}", test);
}
