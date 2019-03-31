use crate::core::csstype::{CssKeyword, CssKeywordType, Cssifiable};
use crate::core::property::Property;

pub struct UserSelect;

impl Property for UserSelect {
  fn register() {
    Self::register_keyword(vec!["none", "auto", "text", "contain", "all"]);

    Self::register_keyword_prefixed("-moz-", vec!["none", "text", "all"]);
    // "all" Doesn't work in Safari; use only "none" or "text", or else it will allow typing in the <html> container
    Self::register_keyword_prefixed("-webkit-", vec!["none", "text", "all"]);
    Self::register_keyword_prefixed("-ms-", vec!["none", "text", "element"]);
  }
  fn name() -> &'static str {
    "user-select"
  }

  fn verify(arg: &Cssifiable) -> bool {
    let arg = arg.as_any();
    if let Some(arg) = arg.downcast_ref::<CssKeyword>() {
      match arg {
        CssKeyword {
          keyword_type: CssKeywordType::NotWide(s),
          ..
        } => match s.as_str() {
          "none" | "auto" | "text" | "contain" | "all" | "element" => true,
          _ => false,
        },
        _ => true,
      }
    } else {
      false
    }
  }
}
