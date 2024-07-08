//! Defines errors used by the lexer
use ecow::EcoString;
use crate::util::Span;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexErrorType {
    StringEscape,                      // unescaped slash
    TrailingUnderscore,                // 100_
    OutOfRadixBounds,                  // 0b12
    NoIntValue,                        // 0x
    UnicodeEscape(UnicodeEscapeError), // \u{-12} (idk)
    UnterminatedString,                // " What the heck! This string isn't terminated!
    UnterminatedComment,               // /* What the heck! This comment isn't ever terminated!
    UnrecognizedToken { tok: char },   // idk
    DisallowedToken { tok: char },     // ; <- we hate these
    BadIdentifier { name: EcoString }, // idk
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnicodeEscapeError {
    MissingLeftBrace,
    ExpectedDigit,
    ExpectedRightBrace,
    NumberOfDigits,
    Codepoint,
}
