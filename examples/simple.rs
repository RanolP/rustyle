#![feature(proc_macro_hygiene)]
use rustyle::{css_use, rustyle};

fn main() {
  #[css_use]
  let red = "gray";

  let test = rustyle! {
    background-color: #00cccc;
    background-color: ${red};
    #[allow(VendorPrefix)]
    -moz-user-select: none;
  };

  println!("{}", test);
}
