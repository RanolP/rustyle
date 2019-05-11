mod border;
mod margin;
mod padding;
mod width;
mod float;
mod height;
mod clear;
mod display;
mod white_space;
mod list_style_type;
mod list_style_position;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(width::Instance);
    registerer.register(height::Instance);
    registerer.register(float::Instance);
    registerer.register(clear::Instance);
    registerer.register(display::Instance);
    registerer.register(white_space::Instance);
    registerer.register(list_style_type::Instance);
    registerer.register(list_style_position::Instance);
    
    margin::register_properties(registerer);
    border::register_properties(registerer);
    padding::register_properties(registerer);
}
