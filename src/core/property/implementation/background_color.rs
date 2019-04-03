use crate::core::csstype::{CssKeyword, CssKeywordType, Cssifiable, HslColor, RgbColor};
use crate::core::property::{register_property, Property};

pub struct BackgroundColor;

impl Property for BackgroundColor {
  fn register(&self) {
    self.register_keyword(vec!["transparent", "currentcolor"]);
    register_property(BackgroundColor);
  }
  fn name(&self) -> &'static str {
    "background-color"
  }

  fn verify(&self, arg: &Cssifiable) -> bool {
    let arg = arg.as_any();
    if let Some(arg) = arg.downcast_ref::<CssKeyword>() {
      match arg {
        CssKeyword {
          keyword_type: CssKeywordType::NotWide(s),
          ..
        } => match s.as_str() {
          "transparent" | "currentcolor" => true,
          _ => false,
        },
        _ => true,
      }
    } else if arg.is::<RgbColor>() {
      true
    } else if arg.is::<HslColor>() {
      true
    } else {
      false
    }
  }
}
