use std::{fmt, fmt::{Debug, Display}};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Codes {
    bits: Vec<bool>
}

impl Codes {
    pub fn from(bits: Vec<bool>) -> Self {
        Self { bits }
    }
}

impl Display for Codes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bin: String = self.bits.iter().map(|b| if *b { "1"} else { "0" }).collect();
        write!(f, "{}", bin)
    }
}
