use crate::core::csstype::{Cssifiable, NumberUnit, PercentageUnit};
use crate::global::{UnitTree, UNIT_TREE};
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
            let mut chars = stringified.chars().rev();
            let mut ch = chars.next().expect("Token does not empty");

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
                        tree = branch;
                    }

                    unit.insert(0, ch);

                    if let Some(c) = chars.next() {
                        ch = c;
                        true
                    } else {
                        false
                    }
                } {}

                if let (Ok(number), UnitTree::Part { end: true, .. }) =
                    (stringified.parse::<f64>(), tree)
                {
                    (stringified, number, Some(unit), span)
                } else {
                    return (None, Some(span));
                }
            }
        }
    } else {
        return (None, None);
    };

    let (origin, unit) = match tokens.peek() {
        Some(TokenTree::Punct(ref punct)) if punct.as_char() == '%' => (
            format!("{}{}", origin, punct.to_string()),
            Some("%".to_string()),
        ),
        _ => (origin, unit),
    };

    if let Some(unit) = unit {
        match unit.as_str() {
            "%" => (
                Some(Box::new(PercentageUnit {
                    origin,
                    val: number,
                })),
                Some(span),
            ),
            _ => {
                span.error(format!("Unit {} is not implemented", unit));
                (None, Some(span))
            }
        }
    } else {
        (
            Some(Box::new(NumberUnit {
                origin,
                val: number,
            })),
            Some(span),
        )
    }
}
