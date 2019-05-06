extern crate csstype;

#[cfg(test)]
mod parse_color {
    use csstype::RgbColor;
    #[test]
    fn must_success() {
        assert_eq!(RgbColor::parse_hex("#000"), Ok(RgbColor { origin: "#000".to_string(), red: 0x00, green: 0x00, blue: 0x00, alpha: 0xff }))
    }
}