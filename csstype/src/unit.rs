use super::Cssifiable;

/// The group of CSS units.
#[derive(Debug, PartialEq, Clone)]
pub enum CssUnitGroup {
    /// Integer like `0`.
    Integer,
    /// Number like `0.1`.
    Number,
    /// Font relative length like `1em`.
    FontRelativeLength,
    /// Viewport relative length like `1vw`.
    ViewportRelativeLength,
    /// Absolute length like `1in`.
    AbsoluteLength,
    /// Angle like `100deg`.
    Angle,
    /// Time like `1s`.
    Time,
    /// Frequency like `1Hz`.
    Frequency,
    /// Resolution like `1ppi`.
    Resolution,
    /// Percentage like `1%`.
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

/// CSS unit value.
#[derive(Debug)]
pub struct CssUnit {
    /// Original text.
    pub origin: String,
    /// The group of this unit.
    pub group: CssUnitGroup,
    /// The number part value.
    pub number: f64,
    /// The unit name.
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
