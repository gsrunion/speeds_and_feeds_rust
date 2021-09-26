use std::fmt::Display;

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
