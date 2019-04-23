use crate::core::csstype::Cssifiable;

#[derive(Debug, PartialEq)]
pub enum CssUnitGroup {
    Integer,
    Number,
    FontRelativeLength,
    ViewportRelativeLength,
    AbsoluteLength,
    Angle,
    Time,
    Frequency,
    Resolution,
    Percentage,
}

// ? I just waste a little time;
/*
pub enum CssUnitType {
    Integer,
    Number,
    Em,
    Ex,
    Cap,
    Ch,
    Ic,
    Rem,
    Lh,
    Rlh,
    Vw,
    Vh,
    Vi,
    Vb,
    Vmin,
    Vmax,
    Cm,
    Mm,
    Q,
    In,
    Pc,
    Pt,
    Px,
    Deg,
    Grad,
    Rad,
    Turn,
    S,
    Ms,
    Hz,
    Khz,
    Dpi,
    Dpcm,
    Dppx,
}
*/

#[derive(Debug)]
pub struct CssUnit {
    pub origin: String,
    pub group: CssUnitGroup,
    pub number: f64,
    pub unit: String,
}

impl Cssifiable for CssUnit {
    fn origin(&self) -> String {
        self.origin.clone()
    }

    fn cssify(&self) -> String {
        format!("{}{}", self.number, self.unit)
    }
}
