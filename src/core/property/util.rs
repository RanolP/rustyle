use super::Parameter;
use crate::core::csstype::{Color, CssKeyword, CssUnit, CssUnitGroup, HslColor, RgbColor};

#[allow(unused)]
pub fn as_color(arg: &Parameter) -> Option<&dyn Color> {
    let arg = arg.as_any();
    arg.downcast_ref::<RgbColor>()
        .map(|it| it as &dyn Color)
        .or_else(|| arg.downcast_ref::<HslColor>().map(|it| it as &dyn Color))
}

#[allow(unused)]
pub fn is_color(arg: &Parameter) -> bool {
    let arg = arg.as_any();
    arg.is::<RgbColor>() || arg.is::<HslColor>()
}

#[allow(unused)]
pub fn as_keyword(arg: &Parameter) -> Option<&CssKeyword> {
    let arg = arg.as_any();
    arg.downcast_ref::<CssKeyword>()
}

#[allow(unused)]
pub fn is_keyword(arg: &Parameter) -> bool {
    let arg = arg.as_any();
    arg.is::<CssKeyword>()
}

#[allow(unused)]
pub fn as_unit(arg: &Parameter) -> Option<&CssUnit> {
    let arg = arg.as_any();
    arg.downcast_ref::<CssUnit>()
}

#[allow(unused)]
pub fn is_unit(arg: &Parameter) -> bool {
    let arg = arg.as_any();
    arg.is::<CssUnit>()
}

#[allow(unused)]
pub fn is_unit_group<F>(arg: &Parameter, filter: F) -> bool
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
pub fn is_length_unit(arg: &Parameter) -> bool {
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
pub fn is_angle_unit(arg: &Parameter) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Angle)
}

#[allow(unused)]
pub fn is_time_unit(arg: &Parameter) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Time)
}

#[allow(unused)]
pub fn is_frequency_unit(arg: &Parameter) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Frequency)
}

#[allow(unused)]
pub fn is_resolution_unit(arg: &Parameter) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Resolution)
}

#[allow(unused)]
pub fn is_percentage_unit(arg: &Parameter) -> bool {
    is_unit_group(arg, |group| *group == CssUnitGroup::Percentage)
}

pub fn when_keyword<F>(param: &Parameter, f: F) -> bool
where
    F: Fn(&CssKeyword) -> bool,
{
    if let Some(param) = as_keyword(param) {
        f(param)
    } else {
        false
    }
}

#[allow(unused)]
pub fn when_size_0<F>(vec: &Vec<Parameter>, f: F) -> bool
where
    F: Fn() -> bool,
{
    if vec.len() == 0 {
        f()
    } else {
        false
    }
}

#[allow(unused)]
pub fn when_size_1<F>(vec: &Vec<Parameter>, f: F) -> bool
where
    F: Fn(&Parameter) -> bool,
{
    if vec.len() == 1 {
        f(&vec.get(0).expect("Guaranteed by before if"))
    } else {
        false
    }
}

#[allow(unused)]
pub fn when_size_2<F>(vec: &Vec<Parameter>, f: F) -> bool
where
    F: Fn(&Parameter, &Parameter) -> bool,
{
    if vec.len() == 2 {
        f(
            &vec.get(0).expect("Guaranteed by before if"),
            &vec.get(1).expect("Guaranteed by before if"),
        )
    } else {
        false
    }
}
