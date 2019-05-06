#[derive(Clone)]
pub struct Keyword {
    pub vendor_prefix: String,
    pub name: String,
}

impl Keyword {
    pub fn Simple(name: &str) -> Keyword {
        Keyword {
            vendor_prefix: "".to_string(),
            name: name.to_string(),
        }
    }
    pub fn SimpleVec(names: Vec<&str>) -> Vec<Keyword> {
        names.into_iter().map(Keyword::Simple).collect()
    }
    pub fn Prefixed(vendor_prefix: &str, name: &str) -> Keyword {
        Keyword {
            vendor_prefix: vendor_prefix.to_string(),
            name: name.to_string(),
        }
    }
    pub fn PrefixedVec(vendor_prefix: &str, names: Vec<&str>) -> Vec<Keyword> {
        names
            .into_iter()
            .map(|name| Keyword::Prefixed(vendor_prefix, name))
            .collect()
    }
}
