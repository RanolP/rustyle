use crate::core::csstype::{Color, HslColor};
use std::cmp::Ordering::Equal;
use std::str::FromStr;

#[derive(Debug)]
pub struct RgbColor {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl Color for RgbColor {
  fn as_rgb(&self) -> RgbColor {
    RgbColor {
      red: self.red,
      green: self.green,
      blue: self.blue,
      alpha: self.alpha,
    }
  }

  fn as_hsl(&self) -> HslColor {
    let red_ranged = self.red as f32 / 255.0;
    let green_ranged = self.green as f32 / 255.0;
    let blue_ranged = self.blue as f32 / 255.0;

    let mut vec = vec![red_ranged, green_ranged, blue_ranged];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    let (min, max) = (vec[0], vec[2]);
    let delta = max - min;

    let hue = if max == min {
      0.0
    } else if red_ranged == max {
      (green_ranged - blue_ranged) / delta
    } else if green_ranged == max {
      2.0 + (blue_ranged - red_ranged) / delta
    } else {
      4.0 + (red_ranged - green_ranged) / delta
    };

    let hue = hue * 60.0;
    let hue = if hue > 360.0 { 360.0 } else { hue };
    let hue = if hue < 0.0 { hue + 360.0 } else { hue };

    let lightness = (min + max) / 2.0;

    let saturation = if max == min {
      0.0
    } else if lightness <= 0.5 {
      delta / (max + min)
    } else {
      delta / (2.0 - max - min)
    };

    HslColor {
      hue: hue,
      saturation: saturation,
      lightness: lightness,
      alpha: self.alpha,
    }
  }
}

pub enum ColorParseError {
  StringEmpty,
  NotAHexColor,
  InvalidHexColor,
}

impl FromStr for RgbColor {
  type Err = ColorParseError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
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

    let parse = |data: &str| u8::from_str_radix(data, 16).expect("guaranteed by before if");

    let mut read = |count: usize| {
      let mut s = String::new();

      for _ in 0..count {
        s.push(color_chars.next().expect("guaranteed by caller"));
      }

      s
    };

    match color.len() {
      3 => Ok(RgbColor {
        red: parse(&read(1)),
        green: parse(&read(1)),
        blue: parse(&read(1)),
        alpha: 0xff,
      }),
      4 => Ok(RgbColor {
        red: parse(&read(1)),
        green: parse(&read(1)),
        blue: parse(&read(1)),
        alpha: parse(&read(1)),
      }),
      6 => Ok(RgbColor {
        red: parse(&read(2)),
        green: parse(&read(2)),
        blue: parse(&read(2)),
        alpha: 0xff,
      }),
      8 => Ok(RgbColor {
        red: parse(&read(2)),
        green: parse(&read(2)),
        blue: parse(&read(2)),
        alpha: parse(&read(2)),
      }),
      _ => Err(ColorParseError::InvalidHexColor)?,
    }
  }
}
