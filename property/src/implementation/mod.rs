mod background;
mod r#box;
mod direction;
mod empty_cells;
mod font;
mod min_height;
mod text;
mod user_select;

use crate::Registerer;

pub fn register_all_properties(registerer: &mut Registerer) {
    registerer.register(user_select::Instance);
    registerer.register(direction::Instance);
    registerer.register(empty_cells::Instance);
    registerer.register(min_height::Instance);

    background::register_properties(registerer);
    font::register_properties(registerer);
    text::register_properties(registerer);
    r#box::register_properties(registerer);
}
