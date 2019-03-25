use proc_macro::Span;

pub trait Node {
  fn name(&self) -> &'static str;

  fn range(&self) -> Option<(Span, Span)> {
    None
  }

  fn generate_code(&self, base_class: &str) -> String {
    self
      .range()
      .map(|(a, b)| a.join(b).unwrap_or(b))
      .unwrap_or(Span::call_site())
      .error(format!(
        "CodeGenerator not implemented for '{}', at css class '{}'",
        self.name(),
        base_class
      ))
      .emit();

    String::new()
  }
}
