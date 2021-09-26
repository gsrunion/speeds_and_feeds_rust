use std::fmt::Display;
use std::convert::TryFrom;

#[derive(Copy, Clone, Display)]
pub enum Aggressiveness {
    Low,
    Medium,
    High
}

impl Aggressiveness {
    pub fn select(self, range: (f64, f64)) -> f64 {
        match self {
            Aggressiveness::Low => range.0,
            Aggressiveness::Medium => (range.0 + range.1) / 2.0f64,
            Aggressiveness::High => range.1
        }
    }
}

impl TryFrom<u8> for Aggressiveness {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Aggressiveness::Low),
            2 => Ok(Aggressiveness::Medium),
            3 => Ok(Aggressiveness::High),
            _ => Err(())
        }
    }
}