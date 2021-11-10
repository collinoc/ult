#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,            // let
    Func,           // func
    Return,         // return
    If,             // if
    Else,           // else
    While,          // while
    For,            // for
    Break,          // break
    Continue,       // continue
    None,           // none
    
    // Ambiguous symbols
    Plus,           // +
    Increment,      // ++
    PlusEquals,     // +=
    Minus,          // -
    Decrement,      // --
    MinusEquals,    // -=
    Bang,           // !
    NotEquals,      // !=
    GreaterThan,    // >
    GreaterEquals,  // >=
    LessThan,       // <
    LessEquals,     // <=
    BinaryAnd,      // &
    LogicalAnd,     // &&
    BinaryOr,       // |
    LogicalOr,      // ||
        
    // Unambiguous symbols
    Assign,         // :=
    Equals,         // ==
    Multiply,       // *
    Divide,         // /
    Modulo,         // %
    Dot,            // .
    Comma,          // ,
    Semicolon,      // ;
    LParen,         // (
    RParen,         // )
    LSquare,        // [
    RSquare,        // ]
    LCurly,         // {
    RCurly,         // }
    Xor,            // ^
    Negate,         // ~

    // Literals
    Identifier(String),
    StringLiteral(String),
    CharacterLiteral(char),
    NumberLiteral(i64),
    DecimalLiteral(f64),
    BooleanLiteral(bool),
}