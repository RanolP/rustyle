use node::MetadataNode;
use std::ops::{Bound, RangeBounds};

pub enum ParameterType {
    Less,
    Matched,
    Over,
}

fn pluralize(i: u32, unit: &str) -> String {
    match i {
        0 => format!("no {}s", unit),
        1 => format!("1 {}", unit),
        _ => format!("{} {}s", i, unit),
    }
}

fn pluralize_range(range: impl RangeBounds<i32>, unit: &str) -> String {
    if range.end_bound() == Bound::Unbounded {
        match range.start_bound() {
            Bound::Unbounded => format!("any {}", unit),
            Bound::Included(start) => pluralize(*start as u32, &*format!("or more {}", unit)),
            Bound::Excluded(start) => pluralize(*start as u32 + 1, &*format!("or more {}", unit)),
        }
    } else if range.start_bound() == Bound::Unbounded {
        match range.end_bound() {
            Bound::Unbounded => format!("any {}", unit),
            Bound::Included(end) => pluralize(*end as u32, &*format!("or less {}", unit)),
            Bound::Excluded(end) => pluralize((end - 1) as u32, &*format!("or less {}", unit)),
        }
    } else if range.start_bound() == range.end_bound() {
        match range.end_bound() {
            Bound::Unbounded => panic!("Never happen"),
            Bound::Included(end) | Bound::Excluded(end) => pluralize(*end as u32, unit),
        }
    } else {
        let start = match range.start_bound() {
            Bound::Unbounded => panic!("Never happen"),
            Bound::Included(value) | Bound::Excluded(value) => value,
        };
        match range.start_bound() {
            Bound::Unbounded => panic!("Never happen"),
            Bound::Included(end) | Bound::Excluded(end) => {
                format!("{} ~ {}", start, pluralize(*end as u32, unit))
            }
        }
    }
}

pub fn check_param_range(
    expected: impl RangeBounds<i32>,
    node: &MetadataNode,
    error_when_over: bool,
) -> ParameterType {
    let param_count = node.parameters.len() as i32;
    if expected.contains(&(param_count)) {
        return ParameterType::Matched;
    }
    if param_count
        < (match expected.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(val) => *val,
            Bound::Excluded(val) => (*val - 1),
        })
    {
        node.range
            .error(format!(
                "{} expected but {} received",
                pluralize_range(expected, "parameter"),
                pluralize(param_count as u32, "parameter")
            ))
            .emit();
        return ParameterType::Less;
    } else {
        if error_when_over {
            node.range
                .error(format!(
                    "{} expected but {} received",
                    pluralize_range(expected, "parameter"),
                    pluralize(param_count as u32, "parameter")
                ))
                .emit();
        } else {
            node.range
                .warning(format!(
                    "{} expected but {} received",
                    pluralize_range(expected, "parameter"),
                    pluralize(param_count as u32, "parameter")
                ))
                .emit();
        }
        return ParameterType::Over;
    }
}

pub fn check_param_exact(
    expected: i32,
    node: &MetadataNode,
    error_when_over: bool,
) -> ParameterType {
    check_param_range(expected..=expected, node, error_when_over)
}

pub fn no_duplicate(node: &MetadataNode) {
    node.range.warning("Consider removing duplicates").emit();
}
