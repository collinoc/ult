#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,                // let
    Func,               // func
    Return,             // return
    If,                 // if
    Else,               // else
    While,              // while
    For,                // for
    Break,              // break
    Continue,           // continue
    
    // Ambiguous symbols
    Plus,               // +
    Increment,          // ++
    PlusEquals,         // +=
    Minus,              // -
    Decrement,          // --
    MinusEquals,        // -=
    Multiply,           // *
    Exponentiate,       // **
    Not,                // !
    NotEquals,          // !=
    GreaterThan,        // >
    GreaterEquals,      // >=
    ShiftRight,         // >>
    LessThan,           // <
    LessEquals,         // <=
    ShiftLeft,          // <<
    BinaryAnd,          // &
    LogicalAnd,         // &&
    BinaryOr,           // |
    LogicalOr,          // ||

    // Unambiguous symbols
    Assign,             // :=
    Equals,             // ==
    Divide,             // /
    Modulo,             // %
    Dot,                // .
    Comma,              // ,
    Semicolon,          // ;
    LeftParenthesis,    // (
    RightParenthesis,   // )
    LeftBracket,        // [
    RightBracket,       // ]
    LeftBrace,          // {
    RightBrace,         // }
    Xor,                // ^
    BinaryNegate,       // ~
                
    // Literals
    Null,               // null
    Identifier(String),
    StringLiteral(String),
    CharacterLiteral(char),
    IntegerLiteral(i64),
    DecimalLiteral(f64),
    BooleanLiteral(bool),
    
    EOF
}

impl Token {
    /// Returns the precedence for a given token
    pub fn prec(&self) -> i32 {
        use Token::*;
        match self {
            // Binary
            Plus                => 1,
            Minus               => 1,

            Multiply            => 2,
            Divide              => 2,
            Modulo              => 2,
            LogicalAnd          => 2,
            LogicalOr           => 2,

            Exponentiate        => 3,
            LessThan            => 3,
            LessEquals          => 3,
            GreaterThan         => 3,
            GreaterEquals       => 3,
            Equals              => 3,
            NotEquals           => 3,
            BinaryAnd           => 3,
            BinaryOr            => 3,
            ShiftRight          => 3,
            ShiftLeft           => 3,

            // Unary
            Xor                 => 4,
            Not                 => 4,
            BinaryNegate        => 4,

            // Other
            LeftParenthesis     => 10,
            RightBracket        => 10,
            Dot                 => 10,

            _ => 0
        }
    }
}