use crate::{util, RuleMetadataProcessor};
use node::{DeclarationNode, MetadataNode};

#[derive(Debug)]
pub struct NoWarn;

struct ShouldWarn {
    vendor_prefix: bool,
}

impl RuleMetadataProcessor for NoWarn {
    fn name(&self) -> &str {
        "no_warn"
    }
    fn process(&self, node: &DeclarationNode, metadatas: Vec<MetadataNode>) {
        let mut warn = ShouldWarn {
            vendor_prefix: true,
        };

        for metadata in metadatas {
            match util::check_param_exact(1, &metadata, false) {
                util::ParameterType::Less => continue,
                _ => {}
            }

            match metadata.parameters[0].as_str() {
                "vendor_prefix" => {
                    if warn.vendor_prefix {
                        warn.vendor_prefix = false;
                    } else {
                        util::no_duplicate(&metadata);
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
            node.range
                .warning("Consider removing the vendor prefix")
                .emit();
        }
    }
}
