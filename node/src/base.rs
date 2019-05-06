use proc_macro::Span;

pub trait Node {
    fn name(&self) -> &str;

    fn span(&self) -> Option<Span> {
        None
    }
}
