use std::fmt::Display;
use std::convert::TryFrom;

#[derive(Copy, Clone, Display)]
pub enum Tool {
    Sixteenth,
    Eighth,
    Quarter,
}

impl Tool {
    pub fn value(self) -> f64 {
        match self {
            Tool::Sixteenth => 1.0f64 / 16.0f64,
            Tool::Eighth => 1.0f64 / 8.0f64,
            Tool::Quarter => 1.0f64 / 4.0f64
        }
    }
}

impl TryFrom<u8> for Tool {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Tool::Sixteenth),
            2 => Ok(Tool::Eighth),
            3 => Ok(Tool::Quarter),
            _ => Err(())
        }
    }
}
