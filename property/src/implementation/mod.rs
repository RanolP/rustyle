mod background_color;
mod clear;
mod color;
mod direction;
mod display;
mod empty_cells;
mod float;
mod font;
mod height;
mod letter_spacing;
mod list_style_position;
mod list_style_type;
mod min_height;
mod user_select;

use crate::Registerer;

pub fn register_all_properties(registerer: &mut Registerer) {
    registerer.register(background_color::Instance);
    registerer.register(user_select::Instance);
    registerer.register(color::Instance);
    registerer.register(list_style_type::Instance);
    registerer.register(list_style_position::Instance);
    registerer.register(clear::Instance);
    registerer.register(direction::Instance);
    registerer.register(display::Instance);
    registerer.register(empty_cells::Instance);
    registerer.register(float::Instance);
    registerer.register(min_height::Instance);
    registerer.register(letter_spacing::Instance);
    registerer.register(height::Instance);

    font::register_properties(registerer);
}
