#![feature(proc_macro_hygiene)]
use rustyle::{css_files, css_use, rustyle};

fn main() {
  #[css_use]
  let red = "#c0ffee";

  let (test, test_file) = rustyle! {
    #![filename(test.css)]
    background-color: #00cccc;
    #[no_warn(vendor_prefix)]
    -moz-user-select: #00cccc;
    // todo: css_use
    // background-color: ${red};
  };

  let (test2, test2_file) = rustyle! {
    #![filename(test.css)]
    background-color: #00cccc;
    #[no_warn(vendor_prefix)]
    -moz-user-select: #00cccc;
    // todo: css_use
    // background-color: ${red};
  };

  println!("test is {} at {}", test, test_file);
  println!("test2 is {} at {}", test2, test2_file);

  println!("All files are listed here:\n{:?}", css_files!());
}
