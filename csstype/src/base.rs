use std::any::Any;
use std::fmt::Debug;

/// A trait can be convert self into a [`Any`] type.
///
/// [`Any`]: std::any::Any
pub trait AsAny {
    /// Convert self to [`Any`]
    fn as_any(&self) -> &dyn Any;
}

/// A trait can be converted to valid css value.
pub trait Cssifiable: AsAny + Any + Debug {
    /// Original text.
    fn origin(&self) -> String;

    /// Cssified value with less converting.
    fn cssify(&self) -> String;

    /// Cssified & Optimized value, maybe loses original meaning.
    fn optimized_cssify(&self) -> String {
        self.cssify()
    }
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
