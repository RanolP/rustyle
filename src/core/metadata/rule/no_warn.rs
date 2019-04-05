use crate::core::metadata::RuleMetadataProcessor;
use crate::core::node::{DeclarationNode, MetadataNode};

#[derive(Debug)]
pub struct NoWarn;

struct ShouldWarn {
  vendor_prefix: bool,
}

impl RuleMetadataProcessor for NoWarn {
  fn name(&self) -> &'static str {
    "no_warn"
  }
  fn process(&self, node: &DeclarationNode, metadatas: Vec<MetadataNode>) {
    let mut warn = ShouldWarn {
      vendor_prefix: true,
    };

    for metadata in metadatas {
      match metadata.parameters.len() {
        0 => {
          metadata
            .range
            .error("one parameter expected but no parameter received")
            .emit();
          continue;
        }
        1 => {}
        _ => {
          metadata
            .range
            .warning("2 or more parameters received")
            .emit();
        }
      }

      match metadata.parameters[0].as_str() {
        "vendor_prefix" => {
          if warn.vendor_prefix {
            warn.vendor_prefix = false;
          } else {
            metadata
              .range
              .warning("Consider removing duplicated no_warn metadata")
              .emit();
          }
        }
        param @ _ => {
          metadata
            .range
            .error(format!("Unexpected parameter {}", param))
            .emit();
        }
      }
    }

    if node.prefix.len() > 0 && warn.vendor_prefix {
      node
        .range
        .warning("Consider removing the vendor prefix")
        .emit();
    }
  }
}
