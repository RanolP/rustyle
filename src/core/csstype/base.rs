use std::fmt::Debug;

pub trait Cssifiable: Debug {
  fn cssify(&self) -> String;

  fn optimized_cssify(&self) -> String {
    self.cssify()
  }
}
