//! Defines errors used by the lexer
use ecow::EcoString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub error: LexErrorType,
    pub location: Span,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexErrorType {
    BadEscape,
    TrailingUnderscore,
    OutOfRadixBounds,
    NoIntValue,
    UnterminatedString,
    UnrecognizedToken { tok: char },
    BadIdentifier { name: EcoString },
} 

