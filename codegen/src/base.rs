use node::Node;
use proc_macro::Span;
use runtime::CompileContext;

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
