use crate::Cssifiable;
use crate::{HslColor, RgbColor};
use std::fmt::Debug;

/// A trait which refers color type of CSS.
pub trait Color: Debug {
    /// Original text.
    fn origin(&self) -> String;

    /// Color alpha value.
    fn alpha(&self) -> u8;

    /// Convert self as a RGB Color.
    fn as_rgb(&self) -> RgbColor;

    /// Convert self as a HSL Color.
    fn as_hsl(&self) -> HslColor;
}

impl<T: Color + 'static> Cssifiable for T {
    fn origin(&self) -> String {
        self.origin()
    }

    fn cssify(&self) -> String {
        let rgb = self.as_rgb();
        let mut result = String::new();
        result.push_str("#");
        result.push_str(&format!("{:02x}", rgb.red));
        result.push_str(&format!("{:02x}", rgb.green));
        result.push_str(&format!("{:02x}", rgb.blue));

        if rgb.alpha != 0xff {
            result.push_str(&format!("{:02x}", rgb.alpha));
        }

        result
    }

    fn optimized_cssify(&self) -> String {
        if self.alpha() == 0 {
            return "#0000".into();
        }
        let cssified = self.cssify();
        let cssified_without_sharp = &cssified[1..];
        // we knew the cssified result(does not including #) has a length 6 or 8
        let (even, odd) = cssified_without_sharp
            .char_indices()
            .partition::<(Vec<(usize, char)>), _>(|(index, _)| index % 2 == 0);

        for ((_, a), (_, b)) in odd.iter().zip(even.iter()) {
            if a != b {
                return cssified;
            }
        }

        let mut result = String::new();

        result.push_str("#");
        result.push_str(&odd.into_iter().map(|(_, c)| c).collect::<String>());

        result
    }
}
