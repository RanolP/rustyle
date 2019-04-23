use crate::core::csstype::{
    Color, CssKeyword, CssUnit, CssUnitGroup, Cssifiable, HslColor, RgbColor,
};

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

#[allow(unused)]
pub fn as_unit(arg: &Box<dyn Cssifiable>) -> Option<&CssUnit> {
    let arg = arg.as_any();
    arg.downcast_ref::<CssUnit>()
}

#[allow(unused)]
pub fn is_unit(arg: &Box<dyn Cssifiable>) -> bool {
    let arg = arg.as_any();
    arg.is::<CssUnit>()
}

#[allow(unused)]
pub fn is_unit_group<F>(arg: &Box<dyn Cssifiable>, filter: F) -> bool
where
    F: Fn(&CssUnitGroup) -> bool,
{
    if let Some(CssUnit { group, .. }) = as_unit(arg) {
        filter(group)
    } else {
        false
    }
}

#[allow(unused)]
pub fn is_length_unit(arg: &Box<dyn Cssifiable>) -> bool {
    is_unit_group(arg, |group| match *group {
        CssUnitGroup::Integer
        | CssUnitGroup::Number
        | CssUnitGroup::AbsoluteLength
        | CssUnitGroup::FontRelativeLength
        | CssUnitGroup::ViewportRelativeLength => true,
        _ => false,
    })
}

#[allow(unused)]
pub fn is_angle_unit(arg: &Box<dyn Cssifiable>) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Angle)
}

#[allow(unused)]
pub fn is_time_unit(arg: &Box<dyn Cssifiable>) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Time)
}

#[allow(unused)]
pub fn is_frequency_unit(arg: &Box<dyn Cssifiable>) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Frequency)
}

#[allow(unused)]
pub fn is_resolution_unit(arg: &Box<dyn Cssifiable>) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Resolution)
}

#[allow(unused)]
pub fn is_percentage_unit(arg: &Box<dyn Cssifiable>) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Percentage)
}
