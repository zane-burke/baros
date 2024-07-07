//! Defines errors used by the lexer
use ecow::EcoString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub error: LexErrorType,
    pub location: Span,
}

impl LexError {
    pub fn new(err_type: LexErrorType, location: Span) -> Self {
        LexError {
            error: err_type,
            location,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexErrorType {
    StringEscape, // unescaped slash
    TrailingUnderscore, // 
    OutOfRadixBounds,
    NoIntValue,
    UnicodeEscape(UnicodeEscapeError),
    UnterminatedString,
    UnrecognizedToken { tok: char },
    DisallowedToken { tok: char },
    BadIdentifier { name: EcoString },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnicodeEscapeError {
    MissingLeftBrace,
    ExpectedDigit,
    ExpectedRightBrace,
    NumberOfDigits,
    Codepoint,
}
