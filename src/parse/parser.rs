use super::super::lex::token::Token;
use super::error::ParseError;
use super::ast::*;
use super::util::*;
use std::{ iter::Peekable, slice::Iter };

pub type ExpressionResult = Result<Expression, ParseError>;
pub type StatementResult = Result<Statement, ParseError>;

#[derive(Clone)]
pub struct Parser<'p> {
    tok: Peekable<Iter<'p, Token>>
}

// TODO list:
// - Fix name mangling solution for scoping
// - Parse if, while, else, for, etc. so that body is just a single statement (which includes blocks)

impl<'p, 's> Parser<'p> {
    pub fn new(tokens: &'p [Token]) -> Parser<'p> {
        Parser {
            tok: tokens.iter().peekable()
        }
    }

    pub fn next(&mut self) -> Result<Token, ParseError> {
        if let Some(next) = self.tok.next() {
            return Ok((*next).clone())
        }

        Err(ParseError::UnexpectedEOF)
    }

    pub fn peek(&mut self) -> Option<Token> {
        if let Some(next) = self.tok.peek() {
            return Some((*next).clone())
        }

        None
    }

    fn maybe(&mut self, token: Token) -> bool {
        if self.peek() == Some(token) {
            return self.next().is_ok()
        }
        false
    }

    fn expect(&mut self, token: Token) -> Result<Token, ParseError> {
        let tok = self.next()?;

        if tok == token {
            return Ok(token)
        }
        
        Err(ParseError::UnexpectedToken(tok))
    }
     
    pub fn parse(&mut self) -> Result<AST, ParseError> {
        let mut ast = AST::new();

        // Parse program 
        while let Some(tok) = self.peek() {
            match tok {
                Token::Func  => ast.push(self.parse_func_decl(&Scope::Global)?),
                Token::Let   => ast.push(self.parse_variable_decl(&Scope::Global)?),
                Token::EOF => break,

                e => return Err(ParseError::UnexpectedToken(e)),
            }

            self.maybe(Token::Semicolon);
        }

        Ok(ast)
    }

    fn parse_func_decl(&mut self, scope: &'s Scope) -> Result<Declaration, ParseError> {
        self.expect(Token::Func)?;

        let identifier = self.parse_identifier(scope)?;
        let name = identifier.name();
        
        let arguments = self.parse_args_decl(&Scope::Local(name))?;

        let body = self.parse_block(&Scope::Local(identifier.name()))?;
        
        let decl = Declaration::Function { identifier, arguments, body };
        
        Ok(decl)
    }
    
    fn parse_block(&mut self, scope: &'s Scope) -> Result<Block, ParseError> {
        self.expect(Token::LeftBrace)?;

        let mut statements = vec![];
        
        while let Some(tok) = self.peek() {
            if tok == Token::RightBrace { break }
            
            statements.push(self.parse_stmt(scope)?);
        }

        self.expect(Token::RightBrace)?;

        Ok(Block::new(scope.clone(), statements))
    }

    fn parse_stmt(&mut self, scope: &'s Scope) -> StatementResult {
        let stmt = match self.peek() {
            Some(expr) if is_expr_start(&expr) => Statement::Expression(self.parse_expr(scope)?),

            Some(Token::Let)    => Statement::Declaration(self.parse_variable_decl(scope)?),
            Some(Token::Func)   => Statement::Declaration(self.parse_func_decl(scope)?),
            Some(Token::If)     => self.parse_if(scope)?,
            Some(Token::While)  => self.parse_while(scope)?,
            Some(Token::For)    => self.parse_for(scope)?,
            Some(Token::Break)  => self.parse_break()?,
            Some(Token::Continue) => self.parse_continue()?,

            Some(Token::LeftBrace) => Statement::Block(self.parse_block(scope)?),

            Some(Token::Return) => Statement::Return(self.parse_return(scope)?),

            Some(e) => return Err(ParseError::UnexpectedToken(e)),

            None => return Err(ParseError::UnexpectedEOF)
        };

        self.maybe(Token::Semicolon); // Doesn't hurt

        Ok(stmt)
    }

    fn parse_break(&mut self) -> StatementResult {
        self.expect(Token::Break)?;
        Ok(Statement::Break)
    }

    fn parse_continue(&mut self) -> StatementResult {
        self.expect(Token::Continue)?;
        Ok(Statement::Continue)
    }

    fn parse_for(&mut self, scope: &'s Scope) -> StatementResult {
        self.expect(Token::For)?;
        self.expect(Token::LeftParenthesis)?;

        let mut for_scope = scope.name();
        for_scope.push_str("_for");
        let for_scope = &Scope::Local(for_scope);

        let variable = match self.peek() {
            Some(Token::Let) => Some(
                Box::new(
                    Statement::Declaration(self.parse_variable_decl(for_scope)?))),
            Some(Token::Identifier(_)) => Some(
                Box::new(
                    Statement::Expression(
                        Expression::Value(self.parse_identifier(for_scope)?)))),
            _ => None
        };
        
        self.expect(Token::Semicolon)?;

        let condition = match self.peek() {
            Some(Token::Semicolon) => None,
            Some(e) if is_expr_start(&e) => Some(self.parse_expr(for_scope)?),
            Some(_) => return Err(ParseError::SyntaxError),
            None => return Err(ParseError::UnexpectedEOF)
        };

        self.expect(Token::Semicolon)?;

        let step = match self.peek() {
            Some(Token::RightParenthesis) => None,
            Some(e) if is_expr_start(&e) => Some(self.parse_expr(for_scope)?),
            Some(_) => return Err(ParseError::SyntaxError),
            None => return Err(ParseError::UnexpectedEOF)
        };
        
        self.expect(Token::RightParenthesis)?;

        let body = self.parse_block(scope)?;

        Ok(Statement::For {
            variable,
            condition,
            step,
            body
        })
    }

    fn parse_if(&mut self, scope: &'s Scope) -> StatementResult {
        self.expect(Token::If)?;
        
        let condition = self.parse_condition(scope)?;

        let mut local_scope = scope.name();
        local_scope.push_str("_if");

        let body = self.parse_block(&Scope::Local(local_scope))?;

        let else_stmt = match self.peek() {
            Some(Token::Else) => Some(Box::new(self.parse_else(scope)?)),
            _ => None
        };

        Ok(Statement::If {
            condition,
            body,
            else_stmt
        })
    }

    fn parse_else(&mut self, scope: &'s Scope) -> StatementResult {
        self.expect(Token::Else)?;

        let stmt = match self.peek() {
            Some(Token::LeftBrace) => Statement::Block(self.parse_block(scope)?),
            Some(Token::If) => self.parse_if(scope)?,
            Some(e) => return Err(ParseError::BadStatement(e)),
            None => return Err(ParseError::UnexpectedEOF)
        };

        Ok(stmt)
    }

    fn parse_while(&mut self, scope: &'s Scope) -> StatementResult {
        self.expect(Token::While)?;

        let condition = self.parse_condition(scope)?;

        let mut local_scope = scope.name();
        local_scope.push_str("_while");

        let body = self.parse_block(&Scope::Local(local_scope))?;

        Ok(Statement::While {
            condition,
            body
        })
    }

    fn parse_condition(&mut self, scope: &'s Scope) -> ExpressionResult {
        self.expect(Token::LeftParenthesis)?;

        let condition = self.parse_expr(scope)?;

        self.expect(Token::RightParenthesis)?;

        Ok(condition)
    }

    fn parse_return(&mut self, scope: &'s Scope) -> ExpressionResult {
        self.expect(Token::Return)?;

        self.parse_expr(scope)
    }

    fn parse_variable_decl(&mut self, scope: &'s Scope) -> Result<Declaration, ParseError> {
        self.expect(Token::Let)?;
        
        let identifier = self.parse_identifier(scope)?;        

        let value = match self.peek() {
            Some(Token::Semicolon) => None,
            Some(Token::Assign) => { 
                self.next()?; 

                match self.peek() {
                    Some(Token::LeftBracket) => Some(self.parse_array_decl(scope)?),
                    None => return Err(ParseError::UnexpectedEOF),
                    _ => Some(self.parse_expr(scope)?),
                }
            },
            _ => return Err(ParseError::BadAssignment)
        };

        let decl = Declaration::Variable { identifier, value, };

        Ok(decl)
    }

    fn parse_args_decl(&mut self, scope: &'s Scope) -> Result<Option<Vec<Identifier>>, ParseError> {
        self.expect(Token::LeftParenthesis)?;

        let mut args = vec![];
        
        if self.peek() != Some(Token::RightParenthesis) {
            // Initial
            args.push(self.parse_identifier(scope)?);
            
            // Args+
            while let Some(tok) = self.peek() {
                if tok == Token::RightParenthesis { break }
                
                self.expect(Token::Comma)?;
                
                let arg = match self.peek() {
                    Some(_) => self.parse_identifier(scope)?,
                    _ => return Err(ParseError::SyntaxError)
                };
                
                args.push(arg);
            }
        }

        self.expect(Token::RightParenthesis)?;

        match args.len() {
            0 => Ok(None),
            _ => Ok(Some(args))
        }
    }    

    fn parse_array_decl(&mut self, scope: &'s Scope) -> ExpressionResult {
        let mut elements = vec![];

        self.expect(Token::LeftBracket)?;

        if self.peek() != Some(Token::RightBracket) {
            // Initial
            elements.push(self.parse_expr(scope)?);
            
            // Args+
            while let Some(tok) = self.peek() {
                if tok == Token::RightBracket { break }
                
                self.expect(Token::Comma)?;
                
                let arg = match self.peek() {
                    Some(_) => self.parse_expr(scope)?,
                    _ => return Err(ParseError::SyntaxError)
                };
                
                elements.push(arg);
            }
        }

        self.expect(Token::RightBracket)?;

        Ok(Expression::Array {
            scope: scope.clone(),
            elements
        })
    }

    fn parse_expr(&mut self, scope: &'s Scope) -> ExpressionResult {
        let mut expr = match self.peek() {
            // Everything should start with smallest piece: unary op
            Some(tok) if is_unop_start(&tok) => self.parse_unary(scope)?,
            Some(e) => return Err(ParseError::BadExpression(e)),
            None => return Err(ParseError::UnexpectedEOF)
        };
        
        // Next is start to binary expr ? parse : return unary expr
        while let Some(next) = self.peek() {
            if !is_binop(&next) { break }
            
            expr = match next {
                Token::Assign => self.parse_reassignment(expr, scope)?,
                _ => self.parse_binary(expr, scope)?
            };
        }

        Ok(expr)
    }

    fn parse_binary(&mut self, lhs: Expression, scope: &'s Scope) -> ExpressionResult {
        let op = self.next()?;
        let mut rhs = self.parse_unary(scope)?;

        if let Some(next) = self.peek() {
            match next.prec() {
                n if n > op.prec() => 
                    rhs = self.parse_binary(rhs, scope)?,
                n if n == op.prec() => {
                    return self.parse_binary(Expression::Binary {
                        operation: op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }, scope);
                },
                _ => ()
            }
        }

        Ok(Expression::Binary {
            operation: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn parse_unary(&mut self, scope: &'s Scope) -> ExpressionResult {
        let prefix  = self.parse_prefix();

        let mut op = self.parse_op(scope)?;

        while let Some(next) = self.peek() {
            if !is_unop_postfix(&next) { break }
            op = self.parse_postfix(op, scope)?;
        }

        if prefix == None {
            return Ok(op)
        }

        Ok(Expression::Unary {
            prefix,
            operand: Box::new(op)
        })
    }

    fn parse_prefix(&mut self) -> Option<Token> {
        match self.peek() {
            Some(u) if is_unop_prefix(&u) => self.next().ok(),
            _ => None
        }
    }

    fn parse_op(&mut self, scope: &'s Scope) -> ExpressionResult {
        use Expression :: { Literal, Value };
        match self.peek() {
            Some(Token::LeftParenthesis)    => self.parse_group(scope),
            Some(lit) if is_literal(&lit)   => Ok(Literal(self.parse_literal()?)),
            Some(ident) if is_ident(&ident) => Ok(Value(self.parse_identifier(scope)?)),
            Some(un) if is_unop_start(&un)  => self.parse_unary(scope),
            Some(e) => Err(ParseError::BadExpression(e)),
            None => Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_postfix(&mut self, target: Expression, scope: &'s Scope) -> ExpressionResult {
        match self.peek() {
            Some(Token::LeftParenthesis) => self.parse_call(target, scope),
            Some(Token::LeftBracket)     => self.parse_index(target, scope),
            Some(e) => Err(ParseError::BadExpression(e)),
            None => Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_identifier(&mut self, scope: &'s Scope) -> Result<Identifier, ParseError> {
        let name = match self.next()? {
            Token::Identifier(id) => id,
            _ => return Err(ParseError::BadAssignment)
        };

        Ok(Identifier::new(name, scope.clone()))
    }

    fn parse_literal(&mut self) -> Result<Literal, ParseError> {
        let lit = match self.next()? {
            Token::Null =>                  Literal::Null,
            Token::StringLiteral(s) =>      Literal::String(s.to_string()),
            Token::CharacterLiteral(c) =>   Literal::Character(c),
            Token::IntegerLiteral(i) =>     Literal::Integer(i),
            Token::DecimalLiteral(d) =>     Literal::Decimal(d),
            Token::BooleanLiteral(b) =>     Literal::Boolean(b),

            e => return Err(ParseError::BadExpression(e))
        };

        Ok(lit) // It's lit
    }

    fn parse_group(&mut self, scope: &'s Scope) -> ExpressionResult {
        self.expect(Token::LeftParenthesis)?;
        
        let group = self.parse_expr(scope)?;
        
        self.expect(Token::RightParenthesis)?;

        Ok(group)
    }

    fn parse_call(&mut self, target: Expression, scope: &'s Scope) -> ExpressionResult {        
        let args = self.parse_args_call(scope)?;

        Ok(Expression::Call {
            target: Box::new(target),
            args
        })
    }

    fn parse_args_call(&mut self, scope: &'s Scope) -> Result<Option<Vec<Expression>>, ParseError> {
        self.expect(Token::LeftParenthesis)?;

        let mut args = vec![];

        if self.peek() != Some(Token::RightParenthesis) {
            // Initial
            args.push(self.parse_expr(scope)?);
            
            // Args+
            while let Some(tok) = self.peek() {
                if tok == Token::RightParenthesis { break }
                
                self.expect(Token::Comma)?;
                
                let arg = match self.peek() {
                    Some(_) => self.parse_expr(scope)?,
                    _ => return Err(ParseError::SyntaxError)
                };
                
                args.push(arg);
            }
        }

        self.expect(Token::RightParenthesis)?;

        match args.len() {
            0 => Ok(None),
            _ => Ok(Some(args))
        }
    }

    fn parse_index(&mut self, target: Expression, scope: &'s Scope) -> ExpressionResult {
        self.expect(Token::LeftBracket)?;

        let idx = self.parse_expr(scope)?;

        self.expect(Token::RightBracket)?;

        Ok(Expression::Member {
            target: Box::new(target),
            property: Box::new(idx)
        })
    }   

    fn parse_reassignment(&mut self, lhs: Expression, scope: &'s Scope) -> ExpressionResult {
        // Only allow assignment to identifiers
        match lhs {
            Expression::Value(Identifier {..}) => (),
            _ => return Err(ParseError::BadAssignment)
        }

        self.expect(Token::Assign)?;

        let rhs = self.parse_expr(scope)?;

        Ok(Expression::Assignment {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        })
    }
}