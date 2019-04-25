mod font_kerning;
mod font_size;
mod font_stretch;
mod font_style;
mod font_variant_caps;
mod font_variant_position;

use crate::core::property::Property;

pub fn register_properties() {
    font_kerning::Instance.register();
    font_stretch::Instance.register();
    font_style::Instance.register();
    font_variant_caps::Instance.register();
    font_variant_position::Instance.register();
    font_size::Instance.register();
}
