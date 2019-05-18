use crate::{Color, RgbColor};

/// CSS [`Color`] which stores color by HSL system.
///
/// [`Color`]: crate::color::Color
#[derive(Debug)]
pub struct HslColor {
    /// Original text.
    pub origin: String,
    /// Hue.
    pub hue: f32,
    /// Saturation.
    pub saturation: f32,
    /// Lightness.
    pub lightness: f32,
    /// Alpha.
    pub alpha: u8,
}

impl Color for HslColor {
    fn origin(&self) -> String {
        self.origin.clone()
    }

    fn alpha(&self) -> u8 {
        self.alpha
    }

    fn as_rgb(&self) -> RgbColor {
        panic!("Not implemented")
    }

    fn as_hsl(&self) -> HslColor {
        HslColor {
            origin: self.origin(),
            hue: self.hue,
            saturation: self.saturation,
            lightness: self.lightness,
            alpha: self.alpha,
        }
    }
}
