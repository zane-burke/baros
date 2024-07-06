//! Defines Tokens

use ecow::EcoString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    // Control Sequences
    EOF, // End of file
    Newline,
    Undefined,

    Identifier { name: EcoString },
    DocComment { value: String },
    Comment,

    // Types
    Int { value: EcoString },   // Integer literals
    Float { value: EcoString }, // Float literals
    Str { value: EcoString },   // String literals
    Bool { value: EcoString },  // Boolean literals

    // Keywords
    Alias,    // alias
    As,       // as
    Async,    // async
    Await,    // await
    Const,    // const
    Default,  // default
    Do,       // do
    Dynamic,  // dyn
    Enum,     // enum
    Except,   // except
    Function, // fun
    Impl,     // impl
    Import,   // import
    Let,      // let
    Mut,      // mut
    Pub,      // pub
    Return,   // return
    Self_,    // self
    Static,   // static
    Struct,   // struct
    Super,    // super
    Test,     // test
    Trait,    // trait
    Try,      // try
    Type,     // type
    Union,    // union
    Use,      // use
    Var,      // var
    Where,    // where

    // Control Flow
    If,    // if
    Elif,  // elif
    Else,  // else
    Match, // match

    // Looping
    Loop,     // loop
    For,      // for
    While,    // while
    Continue, // continue
    Break,    // break
    In,       // in

    // Binary Operators
    Plus,          // +
    Minus,         // -
    Star,          // *
    Slash,         // /
    Percent,       // %
    DoubleStar,    // **
    DoubleSlash,   // //
    DoublePercent, // %%
    TripleSlash,   // ///

    // Logical Operators
    Equality,   // ==
    Inequality, // !=
    Identity,   // ===
    Less,       // <
    Greater,    // >
    LessEq,     // <=
    GreaterEq,  // >=
    Amp,        // &
    Bar,        // |
    Caret,      // ^
    ShiftRight, // >>
    ShiftLeft,  // <<

    // Assignment Operators
    Eq,              // =
    PlusEq,          // +=
    MinusEq,         // -=
    StarEq,          // *=
    SlashEq,         // /=
    PercentEq,       // %=
    DoublePercentEq, // %%=
    AmpEq,           // &=
    PipeEq,          // |=
    CaretEq,         // ^=
    ShiftRightEq,    // >>=
    ShiftLeftEq,     // <<=

    // Ranges
    UnincRange, // ...
    IncRange,   // =.=
    RightRange, // .=
    LeftRange,  // =..
    DoubleDot,  // ..

    // Punctuation
    Comma,       // ,
    Dot,         // .
    Colon,       // :
    SemiColon,   // ;
    DoubleColon, // ::
    RightArrow,  // ->
    LeftArrow,   // <-
    Question,    // ?
    Hash,        // #
    Dollar,      // $
    Tilde,       // ~
    At,          // @
    Underscore,  // _
    OpenMulti,   // /*
    CloseMulti,  // */

    // Delimiters
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )

    // Reserved Operators (Unused but still parsed as language tokens)
    LessColon,                // <:
    GreaterColon,             // >:
    ColonGreater,             // :>
    ColonLess,                // :<
    LongRightArrow,           // -->
    LongLeftArrow,            // <--
    DoubleRightArrow,         // ->>
    DoubleLeftArrow,          // <<-
    BarredRightArrow,         // |->
    BarredLeftArrow,          // <-|
    CurlyRightArrow,          // ~>
    CurlyLeftArrow,           // <~
    DivotedRightArrow,        // >->
    DivotedLeftArrow,         // <-<
    BidirectionalArrow,       // <->
    InvertedRightArrow,       // -<
    InvertedLeftArrow,        // >-
    InvertedDoubleRightArrow, // -<<
    InvertedDoubleLeftArrow,  // >>-
    LongCurlyRightArrow,      // ~~>
    LongCurlyLeftArrow,       // <~~
    RightPipe,                // |>
    LeftPipe,                 // <|
}

impl Token {
    /// Determines if the given Token has a reserved keyword
    pub fn is_reserved_ident(&self) -> bool {
        match self {
            Token::Alias
            | Token::As
            | Token::Async
            | Token::Await
            | Token::Const
            | Token::Do
            | Token::Dynamic
            | Token::Default
            | Token::Enum
            | Token::Except
            | Token::Function
            | Token::Impl
            | Token::Import
            | Token::Let
            | Token::Mut
            | Token::Pub
            | Token::Return
            | Token::Self_
            | Token::Static
            | Token::Struct
            | Token::Super
            | Token::Test
            | Token::Trait
            | Token::Try
            | Token::Type
            | Token::Union
            | Token::Use
            | Token::Var
            | Token::Where
            | Token::If
            | Token::Elif
            | Token::Else
            | Token::Match
            | Token::Loop
            | Token::For
            | Token::While
            | Token::Continue
            | Token::Break
            | Token::In => true,

            _ => false,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::EOF => "EOF",
            Token::Newline => "NEWLINE",
            Token::Undefined => "UNDEFINED",
            Token::Identifier { name } => name.as_str(),
            Token::DocComment { .. } => "///",
            Token::Comment => "//",
            Token::Int { value }
            | Token::Float { value }
            | Token::Str { value }
            | Token::Bool { value } => value.as_str(),
            Token::Alias => "alias",
            Token::As => "as",
            Token::Async => "async",
            Token::Await => "await",
            Token::Const => "const",
            Token::Default => "default",
            Token::Do => "do",
            Token::Dynamic => "dyn",
            Token::Enum => "enum",
            Token::Except => "except",
            Token::Function => "fun",
            Token::Impl => "impl",
            Token::Import => "import",
            Token::Let => "let",
            Token::Mut => "mut",
            Token::Pub => "pub",
            Token::Return => "return",
            Token::Self_ => "self",
            Token::Static => "static",
            Token::Struct => "struct",
            Token::Super => "super",
            Token::Test => "test",
            Token::Trait => "trait",
            Token::Try => "try",
            Token::Type => "type",
            Token::Union => "union",
            Token::Use => "use",
            Token::Var => "var",
            Token::Where => "where",
            Token::If => "if",
            Token::Elif => "elif",
            Token::Else => "else",
            Token::Match => "match",
            Token::Loop => "loop",
            Token::For => "for",
            Token::While => "while",
            Token::Continue => "continue",
            Token::Break => "break",
            Token::In => "in",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Star => "*",
            Token::Slash => "/",
            Token::Percent => "%",
            Token::DoubleStar => "**",
            Token::DoubleSlash => "//",
            Token::DoublePercent => "%%",
            Token::TripleSlash => "///",
            Token::Equality => "==",
            Token::Inequality => "!=",
            Token::Identity => "===",
            Token::Less => "<",
            Token::Greater => ">",
            Token::LessEq => "<=",
            Token::GreaterEq => ">=",
            Token::Amp => "&",
            Token::Bar => "|",
            Token::Caret => "^",
            Token::ShiftRight => ">>",
            Token::ShiftLeft => "<<",
            Token::Eq => "=",
            Token::PlusEq => "+=",
            Token::MinusEq => "-=",
            Token::StarEq => "*=",
            Token::SlashEq => "/=",
            Token::PercentEq => "%=",
            Token::DoublePercentEq => "%%=",
            Token::AmpEq => "&=",
            Token::PipeEq => "|=",
            Token::CaretEq => "^=",
            Token::ShiftRightEq => ">>=",
            Token::ShiftLeftEq => "<<=",
            Token::UnincRange => "...",
            Token::IncRange => "=.=",
            Token::RightRange => "..=",
            Token::LeftRange => "=..",
            Token::DoubleDot => "..",
            Token::Comma => ",",
            Token::Dot => ".",
            Token::Colon => ":",
            Token::SemiColon => ";",
            Token::DoubleColon => "::",
            Token::RightArrow => "->",
            Token::LeftArrow => "<-",
            Token::Question => "?",
            Token::Hash => "#",
            Token::Dollar => "$",
            Token::Tilde => "~",
            Token::At => "@",
            Token::Underscore => "_",
            Token::OpenMulti => "/*",
            Token::CloseMulti => "*/",
            Token::LeftBrace => "{",
            Token::RightBrace => "}",
            Token::LeftBracket => "[",
            Token::RightBracket => "]",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::LessColon => "<:",
            Token::GreaterColon => ">:",
            Token::ColonGreater => ":>",
            Token::ColonLess => ":<",
            Token::LongRightArrow => "-->",
            Token::LongLeftArrow => "<--",
            Token::DoubleRightArrow => "->>",
            Token::DoubleLeftArrow => "<<-",
            Token::BarredRightArrow => "|->",
            Token::BarredLeftArrow => "<-|",
            Token::CurlyRightArrow => "~>",
            Token::CurlyLeftArrow => "<~",
            Token::DivotedRightArrow => ">->",
            Token::DivotedLeftArrow => "<-<",
            Token::BidirectionalArrow => "<->",
            Token::InvertedRightArrow => "-<",
            Token::InvertedLeftArrow => ">-",
            Token::InvertedDoubleRightArrow => "-<<",
            Token::InvertedDoubleLeftArrow => ">>-",
            Token::LongCurlyRightArrow => "~~>",
            Token::LongCurlyLeftArrow => "<~~",
            Token::RightPipe => "|>",
            Token::LeftPipe => "<|",
        };

        write!(f, "{s}")
    }
}
