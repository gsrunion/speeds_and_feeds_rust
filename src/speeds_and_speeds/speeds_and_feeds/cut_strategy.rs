use std::fmt::Display;

#[derive(Copy, Clone, Display)]
pub enum CutStrategy {
    WideAndShallow,
    NarrowAndDeep
}
