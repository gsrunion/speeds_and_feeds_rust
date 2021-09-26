use std::fmt::Display;

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
