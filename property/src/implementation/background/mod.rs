mod background_color;
mod background_repeat;
mod color;
mod background_attachment;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(background_color::Instance);
    registerer.register(background_repeat::Instance);
    registerer.register(color::Instance);
    registerer.register(background_attachment::Instance);
}
