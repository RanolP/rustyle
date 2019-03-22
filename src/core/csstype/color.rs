use super::base::Cssifiable;
use std::str::FromStr;

#[derive(Debug)]
pub struct Color {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
  pub alpha: u8,
}

impl Cssifiable for Color {
  fn cssify(&self) -> String {
    let mut result = String::new();
    result.push_str("#");
    result.push_str(&format!("{:02x}", self.red));
    result.push_str(&format!("{:02x}", self.green));
    result.push_str(&format!("{:02x}", self.blue));

    if self.alpha != 0xff {
      result.push_str(&format!("{:02x}", self.alpha));
    }

    result
  }

  fn optimized_cssify(&self) -> String {
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

pub enum ColorParseError {
  StringEmpty,
  NotAHexColor,
  InvalidHexColor,
}

impl FromStr for Color {
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
      3 => Ok(Color {
        red: parse(&read(1)),
        green: parse(&read(1)),
        blue: parse(&read(1)),
        alpha: 0xff,
      }),
      4 => Ok(Color {
        red: parse(&read(1)),
        green: parse(&read(1)),
        blue: parse(&read(1)),
        alpha: parse(&read(1)),
      }),
      6 => Ok(Color {
        red: parse(&read(2)),
        green: parse(&read(2)),
        blue: parse(&read(2)),
        alpha: 0xff,
      }),
      8 => Ok(Color {
        red: parse(&read(2)),
        green: parse(&read(2)),
        blue: parse(&read(2)),
        alpha: parse(&read(2)),
      }),
      _ => Err(ColorParseError::InvalidHexColor)?,
    }
  }
}
