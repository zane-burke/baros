//! The lexer for Baros
use crate::lex::error::LexError;
use crate::lex::token::Token;

/// Lexer
#[derive(Debug)]
pub struct Lexer<T: Iterator<Item = (u32, char)>> {
    characters: T,
    pending: Vec<SpannedToken>,
    current_char: Option<char>,
    next_char: Option<char>,
    current_loc: u32,
    next_loc: u32,
    location: u32,
}

/// Pairs a Token with its start and end position
#[derive(Debug)]
pub struct SpannedToken {
    tok: Token, 
    start: u32,
    end: u32,
}

pub type LexResult = Result<SpannedToken, LexError>;

/// Collapses \r\n and \n into just \n
#[derive(Debug)]
pub struct NewlineHandler<T: Iterator<Item = (u32, char)>> {
    source: T,
    current_char: Option<(u32, char)>,
    next_char: Option<(u32, char)>,
}

impl<T> NewlineHandler<T>
where
    T: Iterator<Item = (u32, char)>,
{
    pub fn new(src: T) -> Self {
        let mut handler = NewlineHandler {
            source: src,
            current_char: None,
            next_char: None,
        };
        let _ = handler.next();
        let _ = handler.next();
        handler
    }

    /// Makes the handler continue to the next two characters
    pub fn advance(&mut self) -> Option<(u32, char)> {
        let res = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.source.next();
        res
    }
}

impl<T> Iterator for NewlineHandler<T> where T: Iterator<Item = (u32, char)> {
    type Item = (u32, char);
    /// Collapses `\r\n` (Windows), `\r` (Mac), and `\n` (Linux) into `\n` then continues along to the next characters
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, '\r')) = self.current_char {
            if let Some((_, '\n')) = self.next_char {
                let _ = self.advance();
                self.current_char = Some((i, '\n'))
            } else {
                self.current_char = Some((i, '\n'))
            }
        }

        self.advance()
    }
}
