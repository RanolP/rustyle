use crate::core::compile_context::CompileContext;
use proc_macro::Span;

pub trait Node {
  fn name(&self) -> &str;

  fn span(&self) -> Option<Span> {
    None
  }

  fn generate_code(&self, base_class: &str, _: &mut CompileContext) -> String {
    self
      .span()
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
