#[derive(Clone)]
pub struct Keyword {
    pub vendor_prefix: String,
    pub name: String,
}

impl Keyword {
    pub fn simple(name: &str) -> Keyword {
        Keyword {
            vendor_prefix: "".to_string(),
            name: name.to_string(),
        }
    }
    pub fn simple_vec(names: Vec<&str>) -> Vec<Keyword> {
        names.into_iter().map(Keyword::simple).collect()
    }
    pub fn prefixed(vendor_prefix: &str, name: &str) -> Keyword {
        Keyword {
            vendor_prefix: vendor_prefix.to_string(),
            name: name.to_string(),
        }
    }
    pub fn prefixed_vec(vendor_prefix: &str, names: Vec<&str>) -> Vec<Keyword> {
        names
            .into_iter()
            .map(|name| Keyword::prefixed(vendor_prefix, name))
            .collect()
    }
}
