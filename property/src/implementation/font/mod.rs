mod font_kerning;
mod font_size;
mod font_stretch;
mod font_style;
mod font_variant;
mod font_variant_caps;
mod font_variant_position;
mod font_weight;

use crate::Registerer;

pub fn register_properties(registerer: &mut Registerer) {
    registerer.register(font_kerning::Instance);
    registerer.register(font_stretch::Instance);
    registerer.register(font_style::Instance);
    registerer.register(font_variant_caps::Instance);
    registerer.register(font_variant_position::Instance);
    registerer.register(font_size::Instance);
    registerer.register(font_variant::Instance);
    registerer.register(font_weight::Instance);
}
