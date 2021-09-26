use std::fmt::Display;
use std::convert::{TryFrom};

#[derive(Copy, Clone, Display)]
pub enum Material {
    Soft,
    Medium,
    Hard
}

impl TryFrom<u8> for Material {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Material::Soft),
            2 => Ok(Material::Medium),
            3 => Ok(Material::Hard),
            _ => Err(())
        }
    }
}