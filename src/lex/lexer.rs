//! The lexer for Baros
//! Needs to be extended to include support for various type suffixes (e.g. 100u8, -12i64, etc.)
//! Current Radix enum may not be ideal for handling errors when unsupported radices are used.
use crate::lex::error::{LexError, LexErrorType, Span};
use crate::lex::token::Token;

/// Stores variants for the allowed radices in the language
#[derive(Debug, Clone, Copy)]
pub enum Radix {
    /// Hexadecimal variant
    Hex,
    /// Decimal variant
    Dec,
    /// Octal variant
    Oct,
    /// Binary variant
    Bin,
    // Catch-all variant for unsupported radices
    Radix(u32),
}

impl Radix {
    pub fn as_num(&self) -> u32 {
        match self {
            Radix::Hex => 16,
            Radix::Dec => 10,
            Radix::Oct => 8,
            Radix::Bin => 2,
            Radix::Radix(b) => *b,
        }
    }

    pub fn as_prefix(&self) -> &str {
        match self {
            Radix::Hex => "0x",
            Radix::Dec => "",
            Radix::Oct => "0o",
            Radix::Bin => "0b",
            Radix::Radix(_) => "",
        }
    }
}
/// Lexer
#[derive(Debug)]
pub struct Lexer<T: Iterator<Item = (u32, char)>> {
    /// Character buffer
    characters: T,
    /// Stored tokens
    queue: Vec<SpannedToken>,
    /// The current character
    c_char: Option<char>,
    /// The next character
    n_char: Option<char>,
    /// The current character's index
    c_pos: u32,
    /// The next character's index
    n_pos: u32,
    /// The index of the lexer itself
    location: u32,
}

/// Pairs a `Token` with its start and end position.
///
/// * `0` - The token to be spanned
/// * `1` - Start of span.
/// * `2` - End of span.
pub type SpannedToken = (Token, u32, u32);
pub type LexResult = Result<SpannedToken, LexError>;

pub fn lexer_from_str(src: &str) -> impl Iterator<Item = LexResult> + '_ {
    let charbuf = src.char_indices().map(|(i, ch)| (i as u32, ch));
    let handler = NewlineHandler::new(charbuf);
    Lexer::new(handler)
}

impl<T> Lexer<T>
where
    T: Iterator<Item = (u32, char)>,
{
    /// Creates a new Lexer from the provided iterator
    pub fn new(input: T) -> Self {
        let mut l = Lexer {
            characters: input,
            queue: Vec::new(),
            c_char: None,
            n_char: None,
            c_pos: 0,
            n_pos: 0,
            location: 0,
        };

        let _ = l.next_char();
        let _ = l.next_char();
        l.location = 0;
        l
    }

    /// Retrieve the next character in the buffer
    fn next_char(&mut self) -> Option<char> {
        let current = self.c_char;
        let next = match self.characters.next() {
            Some((loc, ch)) => {
                // triggers everywhere but EOF
                self.c_pos = self.n_pos;
                self.n_pos = loc;
                Some(ch)
            }
            None => {
                // triggers at EOF
                self.c_pos = self.n_pos;
                self.n_pos += 1;
                None
            }
        };
        self.c_char = self.n_char;
        self.n_char = next;
        current
    }

    /// Retrieves the next token
    pub fn next_token(&mut self) -> LexResult {
        while self.queue.is_empty() {
            todo!();
        }

        Ok(self.queue.remove(0))
    }

    /// Looks at the next character and decides what to do
    fn decide_next(&mut self) -> Result<(), LexError> {
        if let Some(ch) = self.c_char {
            let mut check_sign = false;

            if self.is_name_start(ch) {
                check_sign = true;
                let name = self.lex_name()?;
                self.add_to_queue(name)
            } else if self.is_number_start(ch, self.n_char) {
                check_sign = true;
                let num = self.lex_number()?;
                self.add_to_queue(num);
            } else {
                self.lex_other(ch)?;
            }

            if check_sign {
                // ensures correct parsing of minus sign
                if Some('-') == self.c_char && self.is_number_start('-', self.n_char) {
                    self.eat_single_char(Token::Minus);
                }
            }
        } else {
            // reached EOF
            self.add_to_queue((Token::EOF, self.c_pos, self.c_pos))
        }

        Ok(())
    }

    /// Lexes operators, delimiters, and ranges
    fn lex_other(&mut self, ch: char) -> Result<(), LexError> {
        match ch {
            '+' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::PlusEq, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Plus, start, end));
                    }
                }
            } // +=
            '-' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::MinusEq, start, end));
                    }
                    Some('>') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::RightArrow, start, end));
                    }
                    Some('-') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('>') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::LongRightArrow, start, end));
                            }
                            _ => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                return Err(LexError::new(
                                    LexErrorType::UnrecognizedToken { tok: ch },
                                    Span::new(start, end),
                                ));
                            }
                        }
                    }
                    Some('<') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::InvertedRightArrow, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Minus, start, end));
                    }
                }
            } // -=, ->, -->, -<
            '*' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('*') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::DoubleStar, start, end));
                    }
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::StarEq, start, end));
                    }
                    Some('/') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::CloseMulti, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Star, start, end));
                    }
                }
            } // **, *=, */
            '/' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('/') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('/') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::TripleSlash, start, end));
                            }
                            // If mod comments become a thing, uncomment this code.
                            // Some('!') => {
                            //     let _ = self.next_char();
                            //     let end = self.c_pos;
                            //     self.add_to_queue((Token::ModComment, start, end));
                            // }
                            _ => {
                                let end = self.c_pos;
                                self.add_to_queue((Token::DoubleSlash, start, end));
                            }
                        }
                    }
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::SlashEq, start, end));
                    }
                    Some('*') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::OpenMulti, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Slash, start, end));
                    }
                }
            } // //, ///, /=, /*
            '%' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('%') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('=') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::DoublePercentEq, start, end));
                            }
                            _ => {
                                let end = self.c_pos;
                                self.add_to_queue((Token::DoublePercent, start, end));
                            }
                        }
                    }
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::StarEq, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Star, start, end));
                    }
                }
            } // %%, %=, %%=
            '<' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('>') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::LessGreater, start, end));
                    }
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::Eq, start, end));
                    }
                    Some('<') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('=') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::ShiftLeftEq, start, end));
                            }
                            _ => {
                                let end = self.c_pos;
                                self.add_to_queue((Token::ShiftLeft, start, end));
                            }
                        }
                    }
                    Some('-') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('-') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::LongLeftArrow, start, end));
                            }
                            Some('>') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::BidirectionalArrow, start, end));
                            }
                            _ => {
                                let end = self.c_pos;
                                self.add_to_queue((Token::LeftArrow, start, end));
                            }
                        }
                    }
                    Some(':') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::LessColon, start, end));
                    }
                    Some('~') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::CurlyLeftArrow, start, end));
                    }
                    Some('|') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::LeftPipe, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Less, start, end));
                    }
                }
            } // <>, <=, <<, <<=, <-, <:, <--, <~, <->, <|
            '>' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        // matches for >=
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::GreaterEq, start, end));
                    }
                    Some('>') => {
                        // matches for >> and >>=
                        let _ = self.next_char();

                        match self.c_char {
                            Some('=') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::ShiftRightEq, start, end));
                            }
                            _ => {
                                let end = self.c_pos;
                                self.add_to_queue((Token::ShiftRight, start, end));
                            }
                        }
                    }
                    Some('-') => {
                        // matches for >-
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::InvertedLeftArrow, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Greater, start, end))
                    }
                }
            } // >=, >>, >>=, >-
            '&' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::AmpEq, start, end))
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Amp, start, end));
                    }
                }
            }
            '|' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        // matches for |=
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::BarEq, start, end));
                    }
                    Some('>') => {
                        // matches for |>
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::RightPipe, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Bar, start, end));
                    }
                }
            }
            '^' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        // matches for ^=
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::CaretEq, start, end));
                    }
                    _ => {
                        // matches for ^
                        let end = self.c_pos;
                        self.add_to_queue((Token::Caret, start, end));
                    }
                }
            }
            '=' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('=') => {
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::Identity, start, end));
                            }
                            _ => {
                                let end = self.c_pos;
                                self.add_to_queue((Token::Equality, start, end));
                            }
                        }
                    }
                    Some('.') => {
                        let _ = self.next_char();

                        match self.c_char {
                            Some('=') => {
                                // matches for =.=
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::IncRange, start, end));
                            }
                            Some('.') => {
                                // matches for =..
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::LeftRange, start, end))
                            }
                            _ => {
                                // matches for =.
                                let end = self.c_pos;
                                return Err(LexError::new(
                                    LexErrorType::UnrecognizedToken { tok: ch },
                                    Span::new(start, end),
                                ));
                            }
                        }
                    }
                    Some('>') => {
                        // matches for =>
                        let _ = self.next_char();
                        let end = self.c_pos;
                        return Err(LexError::new(
                            LexErrorType::DisallowedToken { tok: ch },
                            Span::new(start, end),
                        ));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Eq, start, end));
                    }
                }
            } // ==, ===, =.=, =.., =>
            ',' => self.eat_single_char(Token::Comma),
            '.' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('.') => {
                        // matches for .. and ..=
                        let _ = self.next_char();

                        match self.c_char {
                            Some('=') => {
                                // matches for ..=
                                let _ = self.next_char();
                                let end = self.c_pos;
                                self.add_to_queue((Token::RightRange, start, end));
                            }
                            _ => {
                                // matches for ..
                                let end = self.c_pos;
                                self.add_to_queue((Token::Range, start, end));
                            }
                        }
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Dot, start, end));
                    }
                }
            }
            ':' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some(':') => {
                        // matches for |=
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::DoubleColon, start, end));
                    }
                    Some('>') => {
                        // matches for |>
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::ColonGreater, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Colon, start, end));
                    }
                }
            } // ::, :>
            ';' => {
                // Semi-colons are not allowed
                let (start, end) = (self.c_pos, self.c_pos);
                return Err(LexError::new(
                    LexErrorType::DisallowedToken { tok: ch },
                    Span::new(start, end),
                ));
            }
            '!' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('=') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::Inequality, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Exclam, start, end));
                    }
                }
            }
            '?' => self.eat_single_char(Token::Question),
            '#' => self.eat_single_char(Token::Hash),
            '$' => self.eat_single_char(Token::Dollar),
            '~' => {
                let start = self.c_pos;
                let _ = self.next_char();

                match self.c_char {
                    Some('>') => {
                        let _ = self.next_char();
                        let end = self.c_pos;
                        self.add_to_queue((Token::CurlyRightArrow, start, end));
                    }
                    _ => {
                        let end = self.c_pos;
                        self.add_to_queue((Token::Tilde, start, end));
                    }
                }
            } // ~>
            '{' => self.eat_single_char(Token::LeftBrace),
            '}' => self.eat_single_char(Token::RightBrace),
            '[' => self.eat_single_char(Token::LeftBracket),
            ']' => self.eat_single_char(Token::RightBracket),
            '(' => self.eat_single_char(Token::LeftParen),
            ')' => self.eat_single_char(Token::RightParen),
            ch => {
                let (start, end) = (self.c_pos, self.c_pos);
                return Err(LexError::new(
                    LexErrorType::UnrecognizedToken { tok: ch },
                    Span::new(start, end),
                ));
            }
        }

        Ok(())
    }

    fn lex_name(&mut self) -> LexResult {
        todo!()
    }

    fn lex_number(&mut self) -> LexResult {
        let start = self.c_pos;
        let number = if self.c_char == Some('0') {
            match self.n_char {
                Some('x') | Some('X') => {
                    let _ = self.next_char();
                    let _ = self.next_char();
                    self.lex_radix(start, Radix::Hex)?
                }
                Some('o') | Some('O') => {
                    let _ = self.next_char();
                    let _ = self.next_char();
                    self.lex_radix(start, Radix::Oct)?
                }
                Some('b') | Some('B') => {
                    let _ = self.next_char();
                    let _ = self.next_char();
                    self.lex_radix(start, Radix::Bin)?
                }
                _ => self.lex_potential_float(),
            }
        } else {
            self.lex_potential_float()
        };

        if Some('_') == self.c_char {
            Err(LexError::new(
                LexErrorType::TrailingUnderscore,
                Span::new(self.c_pos, self.c_pos),
            ))
        } else {
            Ok(number)
        }
    }

    /// Function to lex strings
    fn lex_string(&mut self) -> LexResult {
        todo!()
    }

    /// Function to lex comments
    fn lex_comment(&mut self) -> SpannedToken {
        todo!()
    }

    /// Function to lex base-10 numbers (Int and Float)
    fn lex_potential_float(&mut self) -> SpannedToken {
        self.lex_any_radix(true)
    }

    /// Lexes only integral values in base 10
    fn lex_integer(&mut self) -> SpannedToken {
        self.lex_any_radix(false)
    }

    /// Can lex both float and integral values
    fn lex_any_radix(&mut self, can_be_float: bool) -> SpannedToken {
        let start = self.c_pos;
        let mut value = String::new();

        // Consumes minus
        if self.c_char == Some('-') {
            value.push(self.next_char().expect("lex_normal negative"))
        }

        // Consumes all digits that could occur after a decimal point
        value.push_str(&self.filter_underscores(Radix::Dec));

        // Floats
        if can_be_float && self.c_char == Some('.') {
            value.push(self.next_char().expect("lex_normal float"));
            value.push_str(&self.filter_underscores(Radix::Dec));

            // Scientific
            if self.c_char == Some('e') || self.c_char == Some('E') {
                // allows both cases of e but regularize to just lowercase
                self.c_char = Some('e');
                value.push(self.next_char().expect("lex_normal scientific"));
                if self.c_char == Some('-') {
                    value.push(self.next_char().expect("lex_normal scientific negative"))
                } else if self.c_char == Some('+') {
                    // skip over +, allowing you to have syntax like 1E+10 AND 1E10, rather than just 1E10 and 1E-10
                    let _ = self.next_char();
                }
            }
            let end = self.c_pos;

            // SpannedToken
            (
                Token::Float {
                    value: value.into(),
                },
                start,
                end,
            )
        } else {
            // occurs when cannot be float or no decimal point is used
            let end = self.c_pos;

            (
                Token::Int {
                    value: value.into(),
                },
                start,
                end,
            )
        }
    }

    /// Lexes an integral value of any valid radix
    fn lex_radix(&mut self, start: u32, radix: Radix) -> LexResult {
        let number = self.filter_underscores(radix);

        if number.is_empty() {
            let loc = self.c_pos - 1;
            Err(LexError::new(LexErrorType::NoIntValue, Span::new(loc, loc)))
        } else if radix.as_num() < 16 && Lexer::<T>::is_valid_digit(self.c_char, radix) {
            let loc = self.c_pos;
            Err(LexError::new(
                LexErrorType::OutOfRadixBounds,
                Span::new(loc, loc),
            ))
        } else {
            let value = format!("{}{}", radix.as_prefix(), number);
            let end = self.c_pos;
            Ok((
                Token::Int {
                    value: value.into(),
                },
                start,
                end,
            ))
        }
    }

    /// Lex dot access for tuples
    fn lex_dot_access(&mut self) {
        loop {
            if Some('.') == self.c_char && matches!(self.n_char, Some('0'..='9')) {
                self.eat_single_char(Token::Dot);
                let number = self.lex_integer();
                self.add_to_queue(number);
            } else {
                break;
            }
        }
    }

    /// Consumes a sequence of numbers within the given radix and removes underscores
    fn filter_underscores(&mut self, radix: Radix) -> String {
        let mut value = String::new();

        loop {
            if let Some(c) = self.consume_number(radix) {
                value.push(c);
            } else if self.c_char == Some('_') && Lexer::<T>::is_valid_digit(self.n_char, radix) {
                value.push('_');
                let _ = self.next_char();
            } else {
                break;
            }
        }

        value
    }

    /// Consumes a digit within the given `radix`
    fn consume_number(&mut self, radix: Radix) -> Option<char> {
        let consume_char = Lexer::<T>::is_valid_digit(self.c_char, radix);

        if consume_char {
            Some(self.next_char().expect("consume_number next char"))
        } else {
            None
        }
    }

    /// Determines if a given digit is a valid digit within the given `radix`
    fn is_valid_digit(ch: Option<char>, radix: Radix) -> bool {
        match radix {
            Radix::Bin | Radix::Oct | Radix::Dec | Radix::Hex => {
                ch.filter(|c| c.is_digit(radix.as_num())).is_some()
            }
            other => panic!("The radix {} is not implemented", other.as_num()),
        }
    }

    /// Tests a `char` to see if it is a valid start to an identifier or reserved keyword
    fn is_name_start(&self, ch: char) -> bool {
        matches!(ch, '@' | '_' | 'a'..='z' | 'A'..='Z')
    }

    /// Tests if a `char` is a valid continuation of an identifier or reserved keyword
    fn is_name_continuation(&self) -> bool {
        self.c_char
            .map(|ch| matches!(ch, '_' | '0'..='9' | 'a'..='z' | 'A'..='Z'))
            .unwrap_or(false)
    }

    /// Tests if a `char` is a valid start to a number
    fn is_number_start(&self, current: char, next: Option<char>) -> bool {
        match current {
            '0'..='9' => true,
            '-' => matches!(next, Some('0'..='9')),
            _ => false,
        }
    }

    /// Consumes a single `char` and adds it to `self.queue`
    fn eat_single_char(&mut self, tok: Token) {
        let start = self.c_pos;
        let _ = self.next_char().expect("eat_char"); // step one position
        let end = self.c_pos;
        self.add_to_queue((tok, start, end))
    }

    /// Add a `Token` to the `queue`
    fn add_to_queue(&mut self, tok: SpannedToken) {
        self.queue.push(tok);
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = (u32, char)>,
{
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();

        match tok {
            Ok((Token::EOF, _, _)) => None, // end of file and stop
            c => Some(c),
        }
    }
}

/// Struct for collapsing `\r\n`, `\r`, and `\n` into `\n`
#[derive(Debug)]
pub struct NewlineHandler<T: Iterator<Item = (u32, char)>> {
    source: T,
    current: Option<(u32, char)>,
    next: Option<(u32, char)>,
}

impl<T> NewlineHandler<T>
where
    T: Iterator<Item = (u32, char)>,
{
    /// Create a new `NewlineHandler`
    pub fn new(src: T) -> Self {
        let mut handler = NewlineHandler {
            source: src,
            current: None,
            next: None,
        };
        let _ = handler.next();
        let _ = handler.next();
        handler
    }

    /// Makes the handler continue to the next two characters
    pub fn advance(&mut self) -> Option<(u32, char)> {
        let res = self.current;
        self.current = self.next;
        self.next = self.source.next();
        res
    }
}

impl<T> Iterator for NewlineHandler<T>
where
    T: Iterator<Item = (u32, char)>,
{
    type Item = (u32, char);
    /// Collapses `\r\n` (Windows), `\r` (Mac), and `\n` (Linux) into `\n` then continues along to the next characters
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((i, '\r')) = self.current {
            if let Some((_, '\n')) = self.next {
                let _ = self.advance();
                self.current = Some((i, '\n'))
            } else {
                self.current = Some((i, '\n'))
            }
        }

        self.advance()
    }
}

/// Utility function to turn a `&str` into its keyword
pub fn str_to_keyword(word: &str) -> Option<Token> {
    match word {
        "alias" => Some(Token::Alias),
        "as" => Some(Token::As),
        "async" => Some(Token::Async),
        "await" => Some(Token::Await),
        "const" => Some(Token::Const),
        "default" => Some(Token::Default),
        "do" => Some(Token::Do),
        "dyn" => Some(Token::Dynamic),
        "enum" => Some(Token::Enum),
        "except" => Some(Token::Except),
        "fun" => Some(Token::Function),
        "impl" => Some(Token::Impl),
        "import" => Some(Token::Import),
        "let" => Some(Token::Let),
        "mut" => Some(Token::Mut),
        "pub" => Some(Token::Pub),
        "return" => Some(Token::Return),
        "self" => Some(Token::Self_),
        "static" => Some(Token::Static),
        "struct" => Some(Token::Struct),
        "super" => Some(Token::Super),
        "test" => Some(Token::Test),
        "trait" => Some(Token::Trait),
        "try" => Some(Token::Try),
        "type" => Some(Token::Type),
        "union" => Some(Token::Union),
        "use" => Some(Token::Use),
        "var" => Some(Token::Var),
        "when" => Some(Token::When),
        "where" => Some(Token::Where),
        "if" => Some(Token::If),
        "elif" => Some(Token::Elif),
        "else" => Some(Token::Else),
        "match" => Some(Token::Match),
        "loop" => Some(Token::Loop),
        "for" => Some(Token::For),
        "while" => Some(Token::While),
        "continue" => Some(Token::Continue),
        "break" => Some(Token::Break),
        "in" => Some(Token::In),
        _ => None,
    }
}
