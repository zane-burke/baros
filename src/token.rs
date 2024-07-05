pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

const KEYWORDS: [(&str, TokenType); 134] = [
    ("fun", TokenType::Function),  // fun
    ("let", TokenType::Let),       // let
    ("var", TokenType::Var),       // var
    ("const", TokenType::Const),   // const
    ("struct", TokenType::Struct), // struct
    ("enum", TokenType::Enum),     // enum
    ("union", TokenType::Union),   // union
    ("pub", TokenType::Pub),       // pub
    ("priv", TokenType::Priv),
    ("trait", TokenType::Trait),     // trait
    ("impl", TokenType::Impl),       // impl
    ("static", TokenType::Static),   // static
    ("import", TokenType::Import),   // include
    ("include", TokenType::Include), // include
    ("package", TokenType::Package), // package
    ("use", TokenType::Use),         // use
    ("alias", TokenType::Alias),
    ("dyn", TokenType::Dynamic), // dyn
    ("async", TokenType::Async), // async
    ("await", TokenType::Await), // await
    ("as", TokenType::As),       // as
    ("type", TokenType::Type),
    ("try", TokenType::Try),
    ("except", TokenType::Except),
    ("do", TokenType::Do),
    ("super", TokenType::Super),
    ("move", TokenType::Move),
    ("return", TokenType::Return),
    ("ref", TokenType::Ref),
    ("deref", TokenType::Deref),
    ("extern", TokenType::Extern),
    ("self", TokenType::Self_),           // self
    ("mut", TokenType::Mut),              // Reserved
    ("if", TokenType::If),                // if
    ("elif", TokenType::Elif),            // elif
    ("else", TokenType::Else),            // else
    ("match", TokenType::Match),          // match
    ("loop", TokenType::Loop),            // loop
    ("for", TokenType::For),              // for
    ("while", TokenType::While),          // while
    ("continue", TokenType::Continue),    // continue
    ("break", TokenType::Break),          // break
    ("in", TokenType::In),                // in
    ("true", TokenType::True),            // true
    ("false", TokenType::False),          // false
    ("+", TokenType::Plus),               // +
    ("-", TokenType::Minus),              // -
    ("*", TokenType::Star),               // *
    ("/", TokenType::Slash),              // /
    ("%", TokenType::Percent),            // %
    ("**", TokenType::DoubleStar),        // **
    ("//", TokenType::DoubleSlash),       // //
    ("%%", TokenType::DoublePercent),     // %%
    ("///", TokenType::TripleSlash),      // ///
    ("==", TokenType::Equality),          // ==
    ("!=", TokenType::Inequality),        // !=
    ("===", TokenType::Identity),         // ===
    ("<", TokenType::Less),               // <
    (">", TokenType::Greater),            // >
    ("<=", TokenType::LessEq),            // <=
    (">=", TokenType::GreaterEq),         // >=
    ("&", TokenType::Amp),                // &
    ("|", TokenType::Bar),                // |
    ("^", TokenType::Caret),              // ^
    (">>", TokenType::ShiftRight),        // >>
    ("<<", TokenType::ShiftLeft),         // <<
    ("=", TokenType::Eq),                 // =
    ("+=", TokenType::PlusEq),            // +=
    ("-=", TokenType::MinusEq),           // -=
    ("*=", TokenType::StarEq),            // *=
    ("/=", TokenType::SlashEq),           // /=
    ("%=", TokenType::PercentEq),         // %=
    ("%%=", TokenType::DoublePercentEq),  // %%=
    ("&=", TokenType::AmpEq),             // &=
    ("|=", TokenType::PipeEq),            // |=
    ("^=", TokenType::CaretEq),           // ^=
    (">>=", TokenType::ShiftRightEq),     // >>=
    ("<<=", TokenType::ShiftLeftEq),      // <<=
    ("...", TokenType::UninclusiveRange), // ...
    ("=.=", TokenType::InclusiveRange),   // =.=
    ("..=", TokenType::RightIncRange),    // ..=
    ("=..", TokenType::LeftIncRange),     // =..
    ("..", TokenType::DoubleDot),         // ..
    (",", TokenType::Comma),
    (".", TokenType::Dot), // .
    (":", TokenType::Colon),
    (";", TokenType::SemiColon),
    ("::", TokenType::DoubleColon),
    ("->", TokenType::RightArrow), // ->
    ("<-", TokenType::LeftArrow),  // <-
    ("?", TokenType::Question),
    ("#", TokenType::Hash),
    ("$", TokenType::Dollar),
    ("~", TokenType::Tilde),
    ("@", TokenType::At),
    ("_", TokenType::Underscore),
    ("/*", TokenType::OpenMulti),  // /*
    ("*/", TokenType::CloseMulti), // */
    ("{", TokenType::LeftBrace),
    ("}", TokenType::RightBrace),
    ("[", TokenType::LeftBracket),
    ("]", TokenType::RightBracket),
    ("(", TokenType::LeftParen),
    (")", TokenType::RightParen),
    ("<:", TokenType::LessColon),                 // <:
    (">:", TokenType::GreaterColon),              // >:
    (":>", TokenType::ColonGreater),              // :>
    (":<", TokenType::ColonLess),                 // :<
    ("-->", TokenType::LongRightArrow),           // -->
    ("<--", TokenType::LongLeftArrow),            // <--
    ("->>", TokenType::DoubleRightArrow),         // ->>
    ("<<-", TokenType::DoubleLeftArrow),          // <<-
    ("|->", TokenType::BarredRightArrow),         // |->
    ("<-|", TokenType::BarredLeftArrow),          // <-|
    ("~>", TokenType::CurlyRightArrow),           // ~>
    ("<~", TokenType::CurlyLeftArrow),            // <~
    (">->", TokenType::DivotedRightArrow),        // >->
    ("<-<", TokenType::DivotedLeftArrow),         // <-<
    ("<->", TokenType::BidirectionalArrow),       // <->
    ("-<", TokenType::InvertedRightArrow),        // -<
    (">-", TokenType::InvertedLeftArrow),         // >-
    ("-<<", TokenType::InvertedDoubleRightArrow), // -<<
    (">>-", TokenType::InvertedDoubleLeftArrow),  // >>-
    ("~~>", TokenType::LongCurlyRightArrow),      // ~~>
    ("<~~", TokenType::LongCurlyLeftArrow),       // <~~
    ("|>", TokenType::RightPipe),                 // |>
    ("<|", TokenType::LeftPipe),                  // <|
    ("||>", TokenType::BarredRightPipe),          // ||>
    ("<||", TokenType::BarredLeftPipe),           // <||
    ("|||>", TokenType::DoubleBarredRightPipe),   // |||>
    ("<|||", TokenType::DoubleBarredLeftPipe),    // <|||
    ("<|>", TokenType::BidirectionalPipe),        // <|>
    ("|-", TokenType::LeftTack),                  // |-
    ("-|", TokenType::RightTack),                 // -|
];

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Control Sequences
    EOF, // End of file
    EOS, // End of statement
    Newline,
    Undefined,

    Identifier(String),
    TestingIdentifier,
    TestingType,
    // Keywords
    Function, // fun
    Let,      // let
    Var,      // var
    Const,    // const
    Struct,   // struct
    Enum,     // enum
    Union,    // union
    Pub,      // pub
    Priv,     // priv
    Trait,    // trait
    Impl,     // impl
    Static,   // static
    Import,   // include
    Include,  // include
    Package,  // package
    Use,      // use
    Alias,    // alias
    Dynamic,  // dyn
    Async,    // async
    Await,    // await
    As,       // as
    Type,
    Try,
    Except,
    Do,
    Super,
    Move,
    Return,
    Ref,
    Deref,
    Extern,
    Self_, // self
    Mut,   // Reserved

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

    // Boolean
    True,  // true
    False, // false

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
    UninclusiveRange, // ...
    InclusiveRange,   // =.=
    RightIncRange,    // .=
    LeftIncRange,     // =..
    DoubleDot,        // ..

    // Punctuation
    Comma,
    Dot, // .
    Colon,
    SemiColon,
    DoubleColon,
    RightArrow, // ->
    LeftArrow,  // <-
    Question,   //
    Hash,       // #
    Dollar,     // $
    Tilde,      // ~
    At,         // @
    Underscore, // _
    OpenMulti,  // /*
    CloseMulti, // */

    // Delimiters
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )

    // Reserved (Unused but still parsed as language terms)
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
    BarredRightPipe,          // ||>
    BarredLeftPipe,           // <||
    DoubleBarredRightPipe,    // |||>
    DoubleBarredLeftPipe,     // <|||
    BidirectionalPipe,        // <|>
    LeftTack,                 // |-
    RightTack,                // -|

    // Types
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
}

pub fn lookup(identifier: String) -> TokenType {
    let ident = identifier.as_str();
    match ident {
        "\0" => TokenType::EOF,
        "\n" => TokenType::Newline,
        "fun" => TokenType::Function,
        "let" => TokenType::Let,
        "var" => TokenType::Var,
        "const" => TokenType::Const,
        "struct" => TokenType::Struct, // struct
        "enum" => TokenType::Enum,     // enum
        "union" => TokenType::Union,   // union
        "pub" => TokenType::Pub,       // pub
        "priv" => TokenType::Priv,
        "trait" => TokenType::Trait,     // trait
        "impl" => TokenType::Impl,       // impl
        "static" => TokenType::Static,   // static
        "import" => TokenType::Import,   // include
        "include" => TokenType::Include, // include
        "package" => TokenType::Package, // package
        "use" => TokenType::Use,         // use
        "alias" => TokenType::Alias,
        "dyn" => TokenType::Dynamic, // dyn
        "async" => TokenType::Async, // async
        "await" => TokenType::Await, // await
        "as" => TokenType::As,       // as
        "type" => TokenType::Type,
        "try" => TokenType::Try,
        "except" => TokenType::Except,
        "do" => TokenType::Do,
        "super" => TokenType::Super,
        "move" => TokenType::Move,
        "return" => TokenType::Return,
        "ref" => TokenType::Ref,
        "deref" => TokenType::Deref,
        "extern" => TokenType::Extern,
        "self" => TokenType::Self_,           // self
        "mut" => TokenType::Mut,              // Reserved
        "if" => TokenType::If,                // if
        "elif" => TokenType::Elif,            // elif
        "else" => TokenType::Else,            // else
        "match" => TokenType::Match,          // match
        "loop" => TokenType::Loop,            // loop
        "for" => TokenType::For,              // for
        "while" => TokenType::While,          // while
        "continue" => TokenType::Continue,    // continue
        "break" => TokenType::Break,          // break
        "in" => TokenType::In,                // in
        "true" => TokenType::True,            // true
        "false" => TokenType::False,          // false
        "+" => TokenType::Plus,               // +
        "-" => TokenType::Minus,              // -
        "*" => TokenType::Star,               // *
        "/" => TokenType::Slash,              // /
        "%" => TokenType::Percent,            // %
        "**" => TokenType::DoubleStar,        // **
        "//" => TokenType::DoubleSlash,       // //
        "%%" => TokenType::DoublePercent,     // %%
        "///" => TokenType::TripleSlash,      // ///
        "==" => TokenType::Equality,          // ==
        "!=" => TokenType::Inequality,        // !=
        "===" => TokenType::Identity,         // ===
        "<" => TokenType::Less,               // <
        ">" => TokenType::Greater,            // >
        "<=" => TokenType::LessEq,            // <=
        ">=" => TokenType::GreaterEq,         // >=
        "&" => TokenType::Amp,                // &
        "|" => TokenType::Bar,                // |
        "^" => TokenType::Caret,              // ^
        ">>" => TokenType::ShiftRight,        // >>
        "<<" => TokenType::ShiftLeft,         // <<
        "=" => TokenType::Eq,                 // =
        "+=" => TokenType::PlusEq,            // +=
        "-=" => TokenType::MinusEq,           // -=
        "*=" => TokenType::StarEq,            // *=
        "/=" => TokenType::SlashEq,           // /=
        "%=" => TokenType::PercentEq,         // %=
        "%%=" => TokenType::DoublePercentEq,  // %%=
        "&=" => TokenType::AmpEq,             // &=
        "|=" => TokenType::PipeEq,            // |=
        "^=" => TokenType::CaretEq,           // ^=
        ">>=" => TokenType::ShiftRightEq,     // >>=
        "<<=" => TokenType::ShiftLeftEq,      // <<=
        "..." => TokenType::UninclusiveRange, // ...
        "=.=" => TokenType::InclusiveRange,   // =.=
        "..=" => TokenType::RightIncRange,    // ..=
        "=.." => TokenType::LeftIncRange,     // =..
        ".." => TokenType::DoubleDot,         // ..
        " =>" => TokenType::Comma,
        "." => TokenType::Dot, // .
        ":" => TokenType::Colon,
        ";" => TokenType::SemiColon,
        "::" => TokenType::DoubleColon,
        "->" => TokenType::RightArrow, // ->
        "<-" => TokenType::LeftArrow,  // <-
        "?" => TokenType::Question,
        "#" => TokenType::Hash,
        "$" => TokenType::Dollar,
        "~" => TokenType::Tilde,
        "@" => TokenType::At,
        "_" => TokenType::Underscore,
        "/*" => TokenType::OpenMulti,  // /*
        "*/" => TokenType::CloseMulti, // */
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "[" => TokenType::LeftBracket,
        "]" => TokenType::RightBracket,
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "<:" => TokenType::LessColon,                 // <:
        ">:" => TokenType::GreaterColon,              // >:
        ":>" => TokenType::ColonGreater,              // :>
        ":<" => TokenType::ColonLess,                 // :<
        "-->" => TokenType::LongRightArrow,           // -->
        "<--" => TokenType::LongLeftArrow,            // <--
        "->>" => TokenType::DoubleRightArrow,         // ->>
        "<<-" => TokenType::DoubleLeftArrow,          // <<-
        "|->" => TokenType::BarredRightArrow,         // |->
        "<-|" => TokenType::BarredLeftArrow,          // <-|
        "~>" => TokenType::CurlyRightArrow,           // ~>
        "<~" => TokenType::CurlyLeftArrow,            // <~
        ">->" => TokenType::DivotedRightArrow,        // >->
        "<-<" => TokenType::DivotedLeftArrow,         // <-<
        "<->" => TokenType::BidirectionalArrow,       // <->
        "-<" => TokenType::InvertedRightArrow,        // -<
        ">-" => TokenType::InvertedLeftArrow,         // >-
        "-<<" => TokenType::InvertedDoubleRightArrow, // -<<
        ">>-" => TokenType::InvertedDoubleLeftArrow,  // >>-
        "~~>" => TokenType::LongCurlyRightArrow,      // ~~>
        "<~~" => TokenType::LongCurlyLeftArrow,       // <~~
        "|>" => TokenType::RightPipe,                 // |>
        "<|" => TokenType::LeftPipe,                  // <|
        "||>" => TokenType::BarredRightPipe,          // ||>
        "<||" => TokenType::BarredLeftPipe,           // <||
        "|||>" => TokenType::DoubleBarredRightPipe,   // |||>
        "<|||" => TokenType::DoubleBarredLeftPipe,    // <|||
        "<|>" => TokenType::BidirectionalPipe,        // <|>
        "|-" => TokenType::LeftTack,                  // |-
        "-|" => TokenType::RightTack,                 // -|
        _ => TokenType::TestingIdentifier,
        //prod version _ => TokenType::Identifier(identifier.to_string()),
    }
}
