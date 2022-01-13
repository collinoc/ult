#![allow(dead_code)]
use super::super::lex::token::Token;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub struct AST {
    program: Vec<Declaration>
}

impl AST {
    pub fn new() -> AST {
        AST {
            program: Vec::<Declaration>::new()
        }
    }

    pub fn push(&mut self, dec: Declaration) {
        self.program.push(dec);
    }

    pub fn get(&self) -> &Self {
        self
    }

    pub fn program(&self) -> &Vec<Declaration> {
        &self.program
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    name:   String,
    scope:  Scope,
}

impl Identifier {
    pub fn new(name: String, scope: Scope) -> Self {
        Self {
            name,
            scope
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn scope(&self) -> Scope {
        self.scope.clone()
    }
}

#[derive(Debug)]
pub enum Declaration {
    Function {
        identifier: Identifier,
        arguments:  Option<Vec<Identifier>>,
        body:       Block,
    },
    Variable {
        identifier: Identifier,
        value:      Option<Expression>
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Value(Identifier),
    Member {
        target: Box<Expression>,
        property: Box<Expression>
    },
    Array {
        scope: Scope,
        elements: Vec<Expression>
    },
    Assignment {
        lhs: Box<Expression>,
        rhs: Box<Expression>
    },
    Call {
        target: Box<Expression>,
        args:   Option<Vec<Expression>>
    },
    Unary {
        prefix:     Option<Token>,
        operand:    Box<Expression>,
    },
    Binary {
        lhs:        Box<Expression>,
        operation:  Token,
        rhs:        Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    If {
        condition: Expression,
        body: Block,
        else_stmt: Option<Box<Statement>>
    },
    While {
        condition: Expression,
        body: Block,
    },
    For {
        variable: Option<Box<Statement>>,
        condition: Option<Expression>,
        step: Option<Expression>,
        body: Block
    },
    Else {
        body: Block
    },
    Break,
    Continue,
    Block(Block),
    Return(Expression),
    Expression(Expression),
    Declaration(Declaration)
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Integer(i64),
    Decimal(f64),
    Character(char),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Scope {
    Global,
    Local(String)
}

impl Scope {
    pub fn name(&self) -> String {
        match self {
            Scope::Global => String::from("Global"),
            Scope::Local(s) => s.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Block {
    scope: Scope,
    statements: Vec<Statement>
}

impl Block {
    pub fn new(scope: Scope, statements: Vec<Statement>) -> Self {
        Block {
            scope,
            statements
        }
    }
}