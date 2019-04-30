use crate::global::{UnitTree, UNIT_TREE};
use csstype::{CssUnit, CssUnitGroup, Cssifiable};
use proc_macro::{Span, TokenTree};
use std::iter::Peekable;

pub fn parse_unit<I>(tokens: &mut Peekable<I>) -> (Option<Box<dyn Cssifiable>>, Option<Span>)
where
    I: Iterator<Item = TokenTree>,
{
    let (origin, number, unit, span) = if let Some(token) = tokens.next() {
        let stringified = token.to_string();
        let span = token.span();

        // Case 1. If it is number it does not have unit, or else not a number.
        if stringified.len() == 1 {
            if let Some(number) = stringified.parse::<f64>().ok() {
                (stringified, number, None, span)
            } else {
                return (None, Some(span));
            }
        } else {
            let mut chars = stringified.chars().rev().peekable();
            let mut ch = *chars.peek().expect("Token does not empty");

            // Case 2. Ends with number means that not ends with unit.
            if ('0'..='9').contains(&ch) {
                if let Some(number) = stringified.parse::<f64>().ok() {
                    (stringified, number, None, span)
                } else {
                    return (None, Some(span));
                }
            } else {
                let mut tree: &UnitTree = &UNIT_TREE;
                let mut unit: String = String::new();

                while {
                    if let Some(branch) = tree.children().get(&ch) {
                        chars.next();
                        unit.insert(0, ch);
                        tree = branch;

                        if let Some(c) = chars.peek().cloned() {
                            ch = c;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } {}

                if let (Ok(number), UnitTree::Part { end: true, .. }) = (
                    chars
                        .collect::<Vec<char>>()
                        .into_iter()
                        .rev()
                        .collect::<String>()
                        .parse::<f64>(),
                    tree,
                ) {
                    (stringified, number, Some(unit), span)
                } else {
                    return (None, Some(span));
                }
            }
        }
    } else {
        return (None, None);
    };

    let (origin, unit) = match tokens.peek().cloned() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == '%' => {
            tokens.next();
            (
                format!("{}{}", origin, punct.to_string()),
                Some("%".to_string()),
            )
        }
        _ => (origin, unit),
    };

    if let Some(unit) = unit {
        let group = match unit.to_lowercase().as_str() {
            "%" => CssUnitGroup::Percentage,
            "em" | "ex" | "cap" | "ch" | "ic" | "rem" | "lh" | "rlh" => {
                CssUnitGroup::FontRelativeLength
            }
            "vw" | "vh" | "vi" | "vb" | "vmin" | "vmax" => CssUnitGroup::ViewportRelativeLength,
            "cm" | "mm" | "q" | "in" | "pt" | "pc" | "px" => CssUnitGroup::AbsoluteLength,
            "deg" | "grad" | "rad" | "turn" => CssUnitGroup::Angle,
            "s" | "ms" => CssUnitGroup::Time,
            "hz" | "khz" => CssUnitGroup::Frequency,
            "dpi" | "dpcm" | "dppx" => CssUnitGroup::Resolution,
            _ => {
                span.error(format!("Unit {} is not implemented", unit));
                return (None, Some(span));
            }
        };
        (
            Some(Box::new(CssUnit {
                group,
                origin,
                number,
                unit,
            })),
            Some(span),
        )
    } else {
        (
            Some(Box::new(CssUnit {
                group: if number.round() == number {
                    CssUnitGroup::Integer
                } else {
                    CssUnitGroup::Number
                },
                origin,
                number,
                unit: "".to_string(),
            })),
            Some(span),
        )
    }
}
