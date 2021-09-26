use std::fmt::Display;
use std::convert::TryFrom;

#[derive(Copy, Clone, Display)]
pub enum CutStrategy {
    WideAndShallow,
    NarrowAndDeep
}

impl TryFrom<u8> for CutStrategy {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CutStrategy::WideAndShallow),
            2 => Ok(CutStrategy::NarrowAndDeep),
            _ => Err(())
        }
    }
}