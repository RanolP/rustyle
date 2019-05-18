use crate::{Color, HslColor};
use std::cmp::Ordering::Equal;

/// CSS [`Color`] which stores color by RGB system.
///
/// [`Color`]: crate::color::Color
#[derive(Debug)]
pub struct RgbColor {
    /// Original text.
    pub origin: String,
    /// Red.
    pub red: u8,
    /// Green.
    pub green: u8,
    /// Blue.
    pub blue: u8,
    /// Alpha
    pub alpha: u8,
}

// `origin` should not be compared.
impl PartialEq for RgbColor {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red
            && self.green == other.green
            && self.blue == other.blue
            && self.alpha == other.alpha
    }
}

impl Color for RgbColor {
    fn origin(&self) -> String {
        self.origin.clone()
    }

    fn alpha(&self) -> u8 {
        self.alpha
    }

    fn as_rgb(&self) -> RgbColor {
        RgbColor {
            origin: self.origin.clone(),

            red: self.red,
            green: self.green,
            blue: self.blue,
            alpha: self.alpha,
        }
    }

    fn as_hsl(&self) -> HslColor {
        let red_ranged = f32::from(self.red) / 255.0;
        let green_ranged = f32::from(self.green) / 255.0;
        let blue_ranged = f32::from(self.blue) / 255.0;

        let mut vec = vec![red_ranged, green_ranged, blue_ranged];
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
        let (min, max) = (vec[0], vec[2]);
        let delta = max - min;

        let hue = if (max - min).abs() < std::f32::EPSILON {
            0.0
        } else if (red_ranged - max).abs() < std::f32::EPSILON {
            (green_ranged - blue_ranged) / delta
        } else if (green_ranged - max).abs() < std::f32::EPSILON {
            2.0 + (blue_ranged - red_ranged) / delta
        } else {
            4.0 + (red_ranged - green_ranged) / delta
        };

        let hue = hue * 60.0;
        let hue = if hue > 360.0 { 360.0 } else { hue };
        let hue = if hue < 0.0 { hue + 360.0 } else { hue };

        let lightness = (min + max) / 2.0;

        let saturation = if (max - min).abs() < std::f32::EPSILON {
            0.0
        } else if lightness <= 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        };

        HslColor {
            origin: self.origin.clone(),
            hue,
            saturation,
            lightness,
            alpha: self.alpha,
        }
    }
}

/// The error type which can be occured when parse hex color.
#[derive(Debug, PartialEq)]
pub enum ColorParseError {
    /// When the string is empty
    StringEmpty,
    /// When the string does not starts with `#` character.
    NotAHexColor,
    /// When the string starts with `#` character but trailing letters are invalid.
    InvalidHexColor,
}

impl RgbColor {
    /// Parses color from the string by hex notation.
    pub fn parse_hex(input: &str) -> Result<RgbColor, ColorParseError> {
        if input.is_empty() {
            Err(ColorParseError::StringEmpty)?
        }
        if input.chars().next().expect("guaranteed by before if") != '#' {
            Err(ColorParseError::NotAHexColor)?
        }

        let color = &input[1..];

        if color.chars().any(|ch| match ch {
            '0'..='9' | 'a'..='f' | 'A'..='F' => false,
            _ => true,
        }) {
            Err(ColorParseError::InvalidHexColor)?
        }

        let mut color_chars = color.chars();

        let mut read = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push(color_chars.next().expect("guaranteed by caller"));
            }

            u8::from_str_radix(&s, 16).expect("guaranteed by caller")
        };

        let dup = |v: u8| v + (v << 4);

        match color.len() {
            3 => Ok(RgbColor {
                origin: input.to_string(),
                red: dup(read(1)),
                green: dup(read(1)),
                blue: dup(read(1)),
                alpha: 0xff,
            }),
            4 => Ok(RgbColor {
                origin: input.to_string(),
                red: dup(read(1)),
                green: dup(read(1)),
                blue: dup(read(1)),
                alpha: dup(read(1)),
            }),
            6 => Ok(RgbColor {
                origin: input.to_string(),
                red: read(2),
                green: read(2),
                blue: read(2),
                alpha: 0xff,
            }),
            8 => Ok(RgbColor {
                origin: input.to_string(),
                red: read(2),
                green: read(2),
                blue: read(2),
                alpha: read(2),
            }),
            _ => Err(ColorParseError::InvalidHexColor)?,
        }
    }
}
