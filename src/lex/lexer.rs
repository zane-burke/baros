//! The lexer for Baros
use crate::lex::error::{LexError, LexErrorType, Span};
use crate::lex::token::Token;

/// Stores variants for the allowed radices in the language
#[derive(Debug)]
pub enum Radix {
    /// Hexadecimal variant
    Hex,
    /// Decimal variant
    Dec,
    /// Octal variant
    Oct,
    /// Binary variant
    Bin,
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
            } else {
                self.lex_other(ch)?;
            }
        } else {
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
                if let Some('=') = self.c_char {
                    // matches for +=
                    let _ = self.next_char();
                    let end = self.c_pos;
                    self.add_to_queue((Token::PlusEq, start, end));
                } else {
                    let end = self.c_pos;
                    self.add_to_queue((Token::Plus, start, end));
                }
            } // +=
            '-' => {} // -=, ->, -->, -<
            '*' => {} // **, *=, */
            '/' => {} // //, ///, /=, /*
            '%' => {} // %%, %=, %%=
            '<' => {} // <>, <=, <<, <<=, <-, <:, <--, <~, <->, <|
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
                    Some('>') => { // matches for >> and >>=
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

                if let Some('=') = self.c_char {
                    // matches for &=
                    let _ = self.next_char();
                    let end = self.c_pos;
                    self.add_to_queue((Token::AmpEq, start, end));
                } else {
                    let end = self.c_pos;
                    self.add_to_queue((Token::Amp, start, end));
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
            '=' => {} // ==, ===, =.=, =.., =>
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

                if let Some('=') = self.c_char {
                    // matches for !=
                    let _ = self.next_char();
                    let end = self.c_pos;
                    self.add_to_queue((Token::Inequality, start, end));
                } else {
                    let end = self.c_pos;
                    self.add_to_queue((Token::Exclam, start, end));
                }
            }
            '?' => self.eat_single_char(Token::Question),
            '#' => self.eat_single_char(Token::Hash),
            '$' => self.eat_single_char(Token::Dollar),
            '~' => {
                let start = self.c_pos;
                let _ = self.next_char();
                if let Some('>') = self.c_char {
                    let _ = self.next_char();
                    let end = self.c_pos;
                    self.add_to_queue((Token::CurlyRightArrow, start, end));
                } else {
                    let end = self.c_pos;
                    self.add_to_queue((Token::Tilde, start, end));
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
        "function" => Some(Token::Function),
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
