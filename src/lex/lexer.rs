use crate::lex::token;

#[derive(Default)]
pub struct Lexer {
    input: String,
    current: usize,
    next: usize,
    character: char,
}

impl Lexer {
    pub fn new(inp: String) -> Self {
        Lexer {
            input: inp.clone(),
            current: 0,
            next: 1,
            character: inp.chars().nth(0).unwrap(),
        }
    }

    /// Moves the lexer to the next character to be parsed
    pub fn next_char(&mut self) {
        // This only supports ASCII characters.
        // Switch to unicode-segmentation crate for unicode support.
        if self.next >= self.input.len() {
            self.character = '\0';
        } else {
            self.character = self.input.chars().nth(self.next).unwrap();
        }

        self.current = self.next;
        self.next += 1;
    }

    /// Allows the lexer to peek at the next character
    pub fn peek(&mut self) -> char {
        if self.next >= self.input.len() {
            return '\0';
        }

        self.input.chars().nth(self.next).unwrap()
    }

    /// Tokenizes the current character
    pub fn next_token(&mut self) -> token::Token {
        use token::{Token, TokenType};

        let tok: Token;
        self.skip_whitespace();

        match self.character {
            '=' => {
                tok = Token::new(TokenType::Eq, "=".to_string());
            }
            '+' => {
                tok = Token::new(TokenType::Plus, "+".to_string());
            }
            ';' => {
                tok = Token::new(TokenType::SemiColon, ";".to_string());
            }
            ',' => {
                tok = Token::new(TokenType::Comma, ",".to_string());
            }
            '(' => {
                tok = Token::new(TokenType::LeftParen, "(".to_string());
            }
            ')' => {
                tok = Token::new(TokenType::RightParen, ")".to_string());
            }
            '{' => {
                tok = Token::new(TokenType::LeftBrace, "{".to_string());
            }
            '}' => {
                tok = Token::new(TokenType::RightBrace, "}".to_string());
            }
            '\0' => {
                tok = Token::new(TokenType::EOF, "".to_string());
            }
            _ => {
                if is_valid_character(self.character) {
                    let literal = self.read_identifier();
                    tok = Token::new(token::lookup(literal.clone()), literal);
                } else {
                    tok = Token::new(TokenType::Undefined, "UNDEFINED".to_string());
                }
            }
        }

        self.next_char();
        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.current;
        while is_valid_character(self.character) {
            self.next_char();
        }

        self.input[pos..(self.current)].to_string()
    }

    pub fn skip_whitespace(&mut self) {
        while self.character == ' ' || self.character == '_' || self.character == '\t' || self.character == '\r' {
            self.next_char();
        }
    }

    pub fn handle_newline() {
        todo!();
    }
}

pub fn is_valid_character(character: char) -> bool {
    if character.is_alphabetic() || character == '_' {
        return true;
    }

    false
}
