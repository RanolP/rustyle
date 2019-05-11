mod border_bottom_width;
mod border_left_width;
mod border_right_width;
mod border_top_width;
mod border_style;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(border_bottom_width::Instance);
    registerer.register(border_left_width::Instance);
    registerer.register(border_right_width::Instance);
    registerer.register(border_top_width::Instance);
    registerer.register(border_style::Instance);
}
