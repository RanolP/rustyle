use metadata::{RootMetadataProcessor, RuleMetadataProcessor};
use property::Property;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub type UnitTreeChildren = HashMap<char, UnitTree>;

#[derive(Debug)]
pub enum UnitTree {
    Root {
        children: UnitTreeChildren,
    },
    Part {
        children: UnitTreeChildren,
        end: bool,
    },
}

impl UnitTree {
    pub fn children(&self) -> &UnitTreeChildren {
        match self {
            UnitTree::Root { children } => children,
            UnitTree::Part { children, .. } => children,
        }
    }
}

impl UnitTree {
    pub fn mid(children: UnitTreeChildren) -> UnitTree {
        UnitTree::Part {
            children,
            end: false,
        }
    }

    pub fn end(children: UnitTreeChildren) -> UnitTree {
        UnitTree::Part {
            children,
            end: true,
        }
    }
}

macro_rules! map {
    ($($k:expr => $v: expr),*) => {
        {
            #[allow(unused_mut)]
            let mut map = HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

lazy_static! {
    pub static ref CSS_ID: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    pub static ref CSS_FILES_MAP: Arc<Mutex<HashMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref KEYWORDS: Arc<Mutex<HashMap<String, HashSet<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref IS_STDLIB_INITIALIZED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    pub static ref PROPERTIES: Arc<Mutex<HashMap<String, Box<dyn Property>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    pub static ref RULE_METADATA_PROCESSORS: Arc<
        Mutex<HashMap<String, Box<dyn RuleMetadataProcessor>>>
    > = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ROOT_METADATA_PROCESSORS: Arc<
        Mutex<HashMap<String, Box<dyn RootMetadataProcessor>>>
    > = Arc::new(Mutex::new(HashMap::new()));

    pub static ref OUTPUT: String =
        std::env::var("RUSTYLE_OUTPUT").unwrap_or(String::from("./rustyle"));

    pub static ref UNIT_TREE: UnitTree = UnitTree::Root {
        children: map!(
            'b' => UnitTree::mid(map!(
                // vb: 1% of viewport’s size in the root element’s block axis
                'v' => UnitTree::end(map!())
            )),
            'c' => UnitTree::mid(map!(
                // ic: average character advance of a fullwidth glyph in the element’s font, as represented by the “水” (CJK water ideograph, U+6C34) glyph
                'i' => UnitTree::end(map!()),
                // pc: picas; 1pc = 1/6th of 1in
                'p' => UnitTree::end(map!())
            )),
            'd' => UnitTree::mid(map!(
                'a' => UnitTree::mid(map!(
                    // rad: Radians. There are 2π radians in a full circle.
                    'r' => UnitTree::end(map!(
                        // grad: Gradians, also known as "gons" or "grades". There are 400 gradians in a full circle.
                        'g' => UnitTree::end(map!())
                    ))
                ))
            )),
            'g' => UnitTree::mid(map!(
                'e' => UnitTree::mid(map!(
                    // deg: Degrees. There are 360 degrees in a full circle.
                    'd' => UnitTree::end(map!())
                ))
            )),
            'h' => UnitTree::mid(map!(
                // ch: average character advance of a narrow glyph in the element’s font, as represented by the “0” (ZERO, U+0030) glyph
                'c' => UnitTree::end(map!()),
                // lh: line height of the element
                'l' => UnitTree::end(map!(
                    // rlh: line height of the root element
                    'r' => UnitTree::end(map!())
                )),
                // vh: 1% of viewport’s height
                'v' => UnitTree::end(map!())
            )),
            'i' => UnitTree::mid(map!(
                'p' => UnitTree::mid(map!(
                    // dpi: Dots per inch.
                    'd' => UnitTree::end(map!())
                )),
                // vi: 1% of viewport’s size in the root element’s inline axis
                'v' => UnitTree::end(map!())
            )),
            'm' => UnitTree::mid(map!(
                // em: relative to font size of the element
                'e' => UnitTree::end(map!(
                    // rem: relative to font size of the root element
                    'r' => UnitTree::end(map!())
                )),
                // cm: centimeters; 1cm = 96px/2.54
                'c' => UnitTree::end(map!(
                    'p' => UnitTree::mid(map!(
                        // dpcm: Dots per centimeter.
                        'd' => UnitTree::end(map!())
                    ))
                )),
                // mm: millimeters; 1mm = 1/10th of 1cm
                'm' => UnitTree::end(map!())
            )),
            'n' => UnitTree::mid(map!(
                // in: inches; 1in = 2.54cm = 96px
                'i' => UnitTree::end(map!()),
                'r' => UnitTree::mid(map!(
                    'u' => UnitTree::mid(map!(
                        // turn: Turns. There is 1 turn in a full circle.
                        't' => UnitTree::end(map!())
                    ))
                ))
            )),
            'p' => UnitTree::mid(map!(
                'a' => UnitTree::mid(map!(
                    // cap: cap height (the nominal height of capital letters) of the element’s font
                    'c' => UnitTree::end(map!())
                ))
            )),
            // Q: quarter-millimeters; 1Q = 1/40th of 1cm
            'q' => UnitTree::end(map!()),
            // s: Seconds.
            's' => UnitTree::end(map!(
                // ms: Milliseconds. There are 1000 milliseconds in a second.
                'm' => UnitTree::end(map!())
            )),
            't' => UnitTree::mid(map!(
                // pt: points; 1pt = 1/72th of 1in
                'p' => UnitTree::end(map!())
            )),
            'x' => UnitTree::mid(map!(
                // ex: x-height of the element’s font
                'e' => UnitTree::end(map!()),
                // px: pixels; 1px = 1/96th of 1in
                'p' => UnitTree::end(map!(
                    'p' => UnitTree::mid(map!(
                        // dppx: Dots per px unit.
                        'd' => UnitTree::end(map!())
                    ))
                ))
            )),
            'w' => UnitTree::mid(map!(
                // vw: 1% of viewport’s width
                'v' => UnitTree::end(map!())
            )),
            'z' => UnitTree::mid(map!(
                // ? well, Hz and kHz is standard but not used by any property.
                // Hz: Hertz. It represents the number of occurrences per second.
                'h' => UnitTree::end(map!(
                    // kHz: KiloHertz. A kiloHertz is 1000 Hertz.
                    'k' => UnitTree::end(map!())
                ))
            ))
        )
    };

    // Listed at https://stackoverflow.com/questions/5411026/list-of-css-vendor-prefixes
    pub static ref VENDOR_PREFIXES: Vec<&'static str> = vec![
        "-ms-",     // Microsoft
        "mso-",     // Microsoft Office
        "-moz-",    // Mozilla Foundation
        "-o-",      // Opera Software
        "-xv-",     // Opera Software
        "-atsc-",   // Advanced Television Standards Committee
        "-wap-",    // The WAP Forum
        "-webkit-", // Safari, Chrome (and other WebKit-based browsers)
        "-khtml-",  // Konqueror Browser
        "-konq-",   // Konqueror Browser
        "-apple-",  // Webkit supports properties using the -apple- prefixes as well
        "prince-",  // YesLogic
        "-ah-",     // AntennaHouse
        "-hp-",     // Hewlett Packard
        "-ro-",     // Real Objects
        "-rim-",    // Research In Motion
        "-tc-",     // Tall Components
    ];
}
