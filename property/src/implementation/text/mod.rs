mod letter_spacing;
mod text_align;
mod text_decoration;
mod text_indent;
mod text_transform;
mod vertical_align;
mod word_spacing;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(letter_spacing::Instance);
    registerer.register(word_spacing::Instance);
    registerer.register(text_decoration::Instance);
    registerer.register(vertical_align::Instance);
    registerer.register(text_transform::Instance);
    registerer.register(text_align::Instance);
    registerer.register(text_indent::Instance);
}
