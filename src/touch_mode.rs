use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TouchMode {
    OnlyUI,
    Bezier,
    Circles,
    Diamonds,
    FillDiamonds,
}

impl fmt::Display for TouchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
