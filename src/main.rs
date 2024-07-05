mod token;
mod lexer;

struct Expected<'a> {
    expected_type: token::TokenType,
    expected_literal: &'a str,
}

impl Expected<'_> {
    pub fn new(t: token::TokenType, l: &'static str) -> Self {
        Expected {
            expected_type: t,
            expected_literal: l,
        }
    }
}

fn test_next_token() {
    let input: String = "let five: i32 = 5
    let ten: i32 = 10

    fun add(x: i32, y: i32) i32 {
        x + y
    }

    let res: i32 = add(five, ten)
    ".to_string();

    let tests: [Expected; 40] = [
        Expected::new(token::TokenType::Let, "let"),
        Expected::new(token::TokenType::TestingIdentifier, "five"),
        Expected::new(token::TokenType::Colon, ":"),
        Expected::new(token::TokenType::TestingType, "i32"),
        Expected::new(token::TokenType::Eq, "="),
        Expected::new(token::TokenType::I32, "5"),
        Expected::new(token::TokenType::Let, "let"),
        Expected::new(token::TokenType::TestingIdentifier, "ten"),
        Expected::new(token::TokenType::Colon, ":"),
        Expected::new(token::TokenType::TestingType, "i32"),
        Expected::new(token::TokenType::Eq, "="),
        Expected::new(token::TokenType::I32, "10"),
        Expected::new(token::TokenType::Function, "fun"),
        Expected::new(token::TokenType::TestingIdentifier, "add"),
        Expected::new(token::TokenType::LeftParen, "("),
        Expected::new(token::TokenType::TestingIdentifier, "x"),
        Expected::new(token::TokenType::Colon, ":"),
        Expected::new(token::TokenType::TestingType, "i32"),
        Expected::new(token::TokenType::Comma, ","),
        Expected::new(token::TokenType::TestingIdentifier, "y"),
        Expected::new(token::TokenType::Colon, ":"),
        Expected::new(token::TokenType::TestingType, "i32"),
        Expected::new(token::TokenType::RightParen, ")"),
        Expected::new(token::TokenType::TestingType, "i32"),
        Expected::new(token::TokenType::LeftBrace, "{"),
        Expected::new(token::TokenType::TestingIdentifier, "x"),
        Expected::new(token::TokenType::Plus, "+"),
        Expected::new(token::TokenType::TestingIdentifier, "y"),
        Expected::new(token::TokenType::RightBrace, "}"),
        Expected::new(token::TokenType::Let, "let"),
        Expected::new(token::TokenType::TestingIdentifier, "res"),
        Expected::new(token::TokenType::Comma, ":"),
        Expected::new(token::TokenType::TestingType, "i32"),
        Expected::new(token::TokenType::Eq, "="),
        Expected::new(token::TokenType::TestingIdentifier, "add"),
        Expected::new(token::TokenType::LeftParen, "("),
        Expected::new(token::TokenType::TestingIdentifier, "five"),
        Expected::new(token::TokenType::Comma, ","),
        Expected::new(token::TokenType::TestingIdentifier, "ten"),
        Expected::new(token::TokenType::RightParen, ")"),
    ];

    let mut lex = lexer::Lexer::new(input);

    for (i, test) in tests.iter().enumerate() {
        let tok = lex.next_token();
        
        if tok.token_type != test.expected_type {
           panic!("Test[{}]: Token type is wrong. Expected {:?}, found {:?}", i, test.expected_type, tok.token_type)
        }

        if tok.literal != test.expected_literal {
            panic!("Test[{}]: Literal is wrong. Expected {:?}, found {:?}", i, test.expected_literal, tok.literal)
        }
    }

    println!("Passed test.");
}

fn main() {
    println!("Hello, world!");

    test_next_token();    
}
