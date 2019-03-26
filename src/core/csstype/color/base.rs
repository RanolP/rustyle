use crate::core::csstype::Cssifiable;
use crate::core::csstype::{HslColor, RgbColor};
use std::fmt::Debug;

pub trait Color: Debug {
  fn origin(&self) -> String;

  fn alpha(&self) -> u8;

  fn as_rgb(&self) -> RgbColor;

  fn as_hsl(&self) -> HslColor;
}

impl<T: Color> Cssifiable for T {
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
    let (even, odd): (Vec<(usize, char)>, Vec<(usize, char)>) = cssified_without_sharp
      .char_indices()
      .partition(|(index, _)| index % 2 == 0);

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
