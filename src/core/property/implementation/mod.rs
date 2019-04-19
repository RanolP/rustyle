mod background_color;
mod clear;
mod color;
mod direction;
mod display;
mod list_style_position;
mod list_style_type;
mod user_select;

use super::util;
use crate::core::property::Property;

use background_color::*;
use clear::*;
use color::*;
use direction::*;
use display::*;
use list_style_position::*;
use list_style_type::*;
use user_select::*;

pub fn register_all_properties() {
    BackgroundColor.register();
    UserSelect.register();
    Color.register();
    ListStyleType.register();
    ListStylePosition.register();
    Clear.register();
    Direction.register();
    Display.register();
}
