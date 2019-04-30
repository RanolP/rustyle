use crate::{Color, RgbColor};

#[derive(Debug)]
pub struct HslColor {
    pub origin: String,
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
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
