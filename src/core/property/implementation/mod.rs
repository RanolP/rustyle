mod background_color;
mod user_select;

pub use background_color::*;
pub use user_select::*;

use crate::core::property::Property;

pub fn register_all_properties() {
  BackgroundColor.register();
  UserSelect.register();
}
