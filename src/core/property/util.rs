use crate::core::csstype::{Color, CssKeyword, Cssifiable, HslColor, RgbColor};

#[allow(unused)]
pub fn as_color(arg: &Box<dyn Cssifiable>) -> Option<&dyn Color> {
    let arg = arg.as_any();
    arg.downcast_ref::<RgbColor>()
        .map(|it| it as &dyn Color)
        .or_else(|| arg.downcast_ref::<HslColor>().map(|it| it as &dyn Color))
}

#[allow(unused)]
pub fn is_color(arg: &Box<dyn Cssifiable>) -> bool {
    let arg = arg.as_any();
    arg.is::<RgbColor>() || arg.is::<HslColor>()
}

#[allow(unused)]
pub fn as_keyword(arg: &Box<dyn Cssifiable>) -> Option<&CssKeyword> {
    let arg = arg.as_any();
    arg.downcast_ref::<CssKeyword>()
}

#[allow(unused)]
pub fn is_keyword(arg: &Box<dyn Cssifiable>) -> bool {
    let arg = arg.as_any();
    arg.is::<CssKeyword>()
}
