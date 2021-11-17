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
    Null,               // null
    
    // Ambiguous symbols
    Plus,               // +
    Increment,          // ++
    PlusEquals,         // +=
    Minus,              // -
    Decrement,          // --
    MinusEquals,        // -=
    Not,                // !
    NotEquals,          // !=
    GreaterThan,        // >
    GreaterEquals,      // >=
    LessThan,           // <
    LessEquals,         // <=
    BinaryAnd,          // &
    LogicalAnd,         // &&
    BinaryOr,           // |
    LogicalOr,          // ||

    // Unambiguous symbols
    Assign,             // :=
    Equals,             // ==
    Multiply,           // *
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
    Identifier(String),
    StringLiteral(String),
    CharacterLiteral(char),
    IntegerLiteral(i64),
    DecimalLiteral(f64),
    BooleanLiteral(bool),
}