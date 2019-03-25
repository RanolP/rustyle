use std::fmt::Debug;

pub trait Cssifiable: Debug {
  fn origin(&self) -> String;

  fn cssify(&self) -> String;

  fn optimized_cssify(&self) -> String {
    self.cssify()
  }
}
