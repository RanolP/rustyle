use proc_macro::Span;

pub trait Node {
  fn name(&self) -> &'static str;

  fn generate_code(&self, base_class: &str) -> String {
    Span::call_site()
      .error(format!(
        "CodeGenerator not implemented for '{}', at css class '{}'",
        self.name(),
        base_class
      ))
      .emit();

    String::new()
  }
}
