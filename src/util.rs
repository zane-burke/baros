//! Contains a variety of helpful things that are used throughout the project

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Span { start, end }
    }
}