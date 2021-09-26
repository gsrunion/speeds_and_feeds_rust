use std::fmt::Display;
use std::convert::TryFrom;

#[derive(Copy, Clone, Display)]
pub enum CutType {
    Rough,
    Finish
}

impl TryFrom<u8> for CutType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CutType::Rough),
            2 => Ok(CutType::Finish),
            _ => Err(())
        }
    }
}