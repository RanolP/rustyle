use crate::csstype::Cssifiable;

pub trait Property<Item> {
  fn name() -> &'static str;
  fn verify(arg: Cssifiable) -> bool;
}
