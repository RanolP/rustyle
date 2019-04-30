use std::any::Any;
use std::fmt::Debug;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

pub trait Cssifiable: AsAny + Any + Debug {
    fn origin(&self) -> String;

    fn cssify(&self) -> String;

    fn optimized_cssify(&self) -> String {
        self.cssify()
    }
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
