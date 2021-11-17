use super::token::*;
use super::error::*;
use super::source::*;

type LinePos = (i32, i32);

pub struct Lexer {
    tokens: Vec<Token>,
    source: String,
}

impl<'l> Lexer {
    pub fn new(fname: Option<&String>) -> Lexer {
        if fname.is_none() {
            panic!("No file path given");
        }

        Lexer {
            tokens: vec![],
            source: std::fs::read_to_string(fname.unwrap()).unwrap(),
        }
    }

    pub fn tokens(&'l self) -> &'l Vec<Token> {
        &self.tokens
    }

    pub fn lex(&'l mut self) -> Result<Vec<Token>, TokenError> {
        let mut src = Source::new(&self.source);
        let mut tokens = vec![];

        while src.has_next() {
            match src.next()? {
                '\0' => break,

                '#' => {
                    while src.peek() != '\n' {
                        src.next()?;
                    }
                },

                w if w.is_whitespace() => continue,

                k if is_id_start(k) => {
                    let mut string = String::from(k);

                    while is_id_cont(src.peek()) {
                        string.push(src.next()?);
                    }

                    tokens.push(match_keyword(&string));
                },

                d if d.is_ascii_digit() => {
                    let mut num = String::from(d);

                    while src.peek().is_ascii_digit() ||
                        src.peek() == '.' {
                            num.push(src.next()?);
                    }

                    tokens.push(parse_num(num, src.lp())?);
                },

                '"' => {
                    let mut string = String::new();

                    loop {
                        let c = src.next()?;
                        if c == '"' { break; }

                        string.push(c);
                    }

                    tokens.push(Token::StringLiteral(string));
                }

                '\'' => {
                    let c = src.next()?;

                    if !c.is_ascii() { 
                        return Err(TokenError::Unexpected(c, src.lp())) 
                    }

                    src.expect('\'')?;
                    tokens.push(Token::CharacterLiteral(c));
                }

                ';' => tokens.push(Token::Semicolon),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '%' => tokens.push(Token::Modulo),
                '.' => tokens.push(Token::Dot),
                ',' => tokens.push(Token::Comma),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '(' => tokens.push(Token::LeftParenthesis),
                ')' => tokens.push(Token::RightParenthesis),
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                '^' => tokens.push(Token::Xor),
                '~' => tokens.push(Token::BinaryNegate),

                '=' => {
                    src.expect('=')?;
                    tokens.push(Token::Equals);
                },

                ':' => {
                    src.expect('=')?;
                    tokens.push(Token::Assign);
                },

                '+' => {
                    match src.peek() {
                        '+' => { tokens.push(Token::Increment); src.next()?; },
                        '=' => { tokens.push(Token::PlusEquals); src.next()?; },
                        _ => tokens.push(Token::Plus)
                    }
                },

                '-' => {
                    match src.peek() {
                        '-' => { tokens.push(Token::Decrement); src.next()?; },
                        '=' => { tokens.push(Token::MinusEquals); src.next()?; },
                        _ => tokens.push(Token::Minus)
                    }
                },

                '!' => {
                    match src.peek() {
                        '=' => { tokens.push(Token::NotEquals); src.next()?; },
                        _ => tokens.push(Token::Not)
                    }
                },

                '>' => {
                    match src.peek() {
                        '=' => { tokens.push(Token::GreaterEquals); src.next()?; },
                        _ => tokens.push(Token::GreaterThan)
                    }
                },

                '<' => {
                    match src.peek() {
                        '=' => { tokens.push(Token::LessEquals); src.next()?; },
                        _ => tokens.push(Token::LessThan) 
                    }
                },

                '&' => {
                    match src.peek() {
                        '&' => { tokens.push(Token::LogicalAnd); src.next()?; },
                        _ => tokens.push(Token::BinaryAnd)
                    }
                },

                '|' => {
                    match src.peek() {
                        '|' => { tokens.push(Token::BinaryOr); src.next()?; },
                        _ => tokens.push(Token::LogicalOr)
                    }
                }

                u => return Err(TokenError::Unknown(u, src.lp())),
            }
        }

        self.tokens = tokens.clone();
        Ok(tokens)
    }
}

fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_id_cont(c: char) -> bool {
    is_id_start(c) || c.is_ascii_digit()
}

fn match_keyword(word: &str) -> Token {
    use Token::*;

    match word {
        "let" => Let,
        "func" => Func,
        "return" => Return,
        "if" => If,
        "else" => Else,
        "while" => While,
        "for" => For,
        "break" => Break,
        "continue" => Continue,
        "null" => Null,
        "true" => BooleanLiteral(true),
        "false" => BooleanLiteral(false),

        _ => Identifier(word.into())
    }
}

fn parse_num(num: String, lp: LinePos) -> Result<Token, TokenError> {
    if num.contains('.') {
        match num.parse::<f64>() {
            Ok(n) => Ok(Token::DecimalLiteral(n)),
            Err(_) => Err(TokenError::Invalid(num, lp)),
        }
    } 
    else {
        Ok(Token::IntegerLiteral(num.parse::<i64>().unwrap()))
    }
}