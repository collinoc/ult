use super::super::lex::token::Token;

// Utility functions

pub fn is_expr_start(tok: &Token) -> bool {
    is_unop_start(tok)
}

pub fn is_ident(tok: &Token) -> bool {
    matches!(tok, Token::Identifier(_))
}

pub fn is_literal(tok: &Token) -> bool {
    use Token::*;
    matches!(tok, Null | IntegerLiteral(_) | DecimalLiteral(_) |
        StringLiteral(_) | CharacterLiteral(_) | BooleanLiteral(_))
}

pub fn is_unop_start(tok: &Token) -> bool {
    *tok == Token::LeftParenthesis || is_unop_prefix(tok) || is_ident(tok) || is_literal(tok)
}

// Increment and Decrement operators temporarily unimplemented because they are annoying
pub fn is_unop_prefix(tok: &Token) -> bool {
    use Token::*;
    matches!(tok, BinaryNegate | Minus | Not)
}

pub fn is_unop_postfix(tok: &Token) -> bool {
    use Token::*;
    matches!(tok, LeftParenthesis | LeftBracket)
}

pub fn is_binop(tok: &Token) -> bool {
    use Token::*;
    matches!(tok, Plus | Minus | NotEquals | Equals | GreaterThan | GreaterEquals | LessThan |
        LessEquals | BinaryAnd | LogicalAnd | BinaryOr | LogicalOr | Multiply | Divide | Modulo |
        Xor | ShiftRight | ShiftLeft | Assign)
}

#[allow(dead_code)]
pub fn is_reassignment_op(tok: &Token) -> bool {
    use Token::*;
    matches!(tok, PlusEquals | MinusEquals)
}