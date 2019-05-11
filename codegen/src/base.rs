use node::Node;
use proc_macro::Span;
use runtime::CompileContext;

/// A trait which generates code from itself with some contextual informations.
/// 
/// It separated from [`Node`] because of cycle dependency.
/// 
/// [`Node`]: node::Node
pub trait CodeGenerator: Node {
    fn generate_code(&self, base_class: &str, _: &mut CompileContext) -> String {
        self.span()
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
