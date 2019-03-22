use crate::core::csstype::Cssifiable;
use crate::core::node::Node;

pub struct DeclarationNode {
  pub name: String,
  pub value: Box<Cssifiable>,
}

impl Node for DeclarationNode {
  fn name(&self) -> &'static str {
    "Declaration"
  }

  fn generate_code(&self, _: &str) -> String {
    let value = &*self.value;
    return format!(
      "{key}: {value};",
      key = self.name,
      value = value.optimized_cssify()
    );
  }
}
