#![feature(proc_macro_hygiene)]
use rustyle::{css_files, css_use, rustyle};

fn main() {
  #[css_use]
  let red = "#c0ffee";

  let (test, test_file) = rustyle! {
    background-color: #00aabb;
    #[no_warn(vendor_prefix)]
    -moz-user-select: none;
    background-color: #00bbcc;
    background-color: #"00eeff";
    // todo: css_use
    // background-color: ${red};
  };

  println!("test is {} at {}", test, test_file);

  println!("All files are listed here:\n{:?}", css_files!());
}
