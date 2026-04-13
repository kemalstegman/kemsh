//!

use std::marker::PhantomData;

use crate::{
    abstract_lookahead::ErrorBubbledNLookahead,
    ast::{
        Concrete, ConcreteKind, DeclareLValue, Expression, Identifier, MutableLValue, Operation,
    },
    syntax::lexer::token::{Delimeter, ReservedLexeme, Token},
};

pub struct Parser<I, E>
where
    I: ErrorBubbledNLookahead<2, Token, ParserError<E>>,
{
    iter: I,
    _marker: PhantomData<E>,
}

impl<I, E> Parser<I, E>
where
    I: ErrorBubbledNLookahead<2, Token, ParserError<E>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            _marker: PhantomData,
        }
    }
    pub fn parse_top_level_expression(&mut self) -> Option<Result<Expression, ParserError<E>>> {
        match self.iter.next()? {
            Err(err) => Some(Err(err)),
            Ok(tok) => {
                let expression = match self.parse_expression(tok, Precedence::None) {
                    Ok(expression) => expression,
                    Err(err) => return Some(Err(err)),
                };
                match self.iter.next().ok_or(ParserError::Incomplete).flatten() {
                    Err(err) => Some(Err(err)),
                    Ok(Token::Delimeter(Delimeter::Semicolon)) => Some(Ok(expression)),
                    Ok(tok) => Some(Err(ParserError::Generic {
                        message: format!("expected ; got: {tok:?}"),
                    })),
                }
            }
        }
    }
    pub fn parse_expression(
        &mut self,
        tok: Token,
        precedence: Precedence,
    ) -> Result<Expression, ParserError<E>> {
        let mut left = self.parse_nud(tok)?;
        loop {
            let peeked_token = match self.iter.bubble_peek()? {
                None => break,
                Some(tok) => tok,
            };
            let next_precedence = Precedence::from(peeked_token);
            if precedence > next_precedence
                || (precedence == next_precedence && precedence.is_left_associative())
            {
                break;
            }
            let Some(Ok(op_token)) = self.iter.next() else {
                unreachable!("validated by peek")
            };
            left = self.parse_led(left, op_token)?;
        }
        Ok(left)
    }
    pub fn parse_nud(&mut self, tok: Token) -> Result<Expression, ParserError<E>> {
        match tok {
            Token::Float(n) => Ok(Expression::Concrete(Concrete::Float(n))),
            Token::Integer(n) => Ok(Expression::Concrete(Concrete::Integer(n))),
            Token::LiteralString(s) => Ok(Expression::Concrete(Concrete::String(s))),
            Token::Reserved(ReservedLexeme::True) => {
                Ok(Expression::Concrete(Concrete::Boolean(true)))
            }
            Token::Reserved(ReservedLexeme::False) => {
                Ok(Expression::Concrete(Concrete::Boolean(false)))
            }
            Token::Unreserved(i) => Ok(Expression::Identifier(Identifier(i))),
            Token::Reserved(ReservedLexeme::Let) => Ok(self.parse_nud_let()?),
            Token::Reserved(ReservedLexeme::Exit) => Ok(self.parse_nud_exit()?),
            _ => Err(ParserError::Generic {
                message: format!("unexpected token: {tok:?}"),
            }),
        }
    }
    pub fn parse_nud_let(&mut self) -> Result<Expression, ParserError<E>> {
        let mut tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
        let identifier = match tok {
            Token::Unreserved(identifier) => Identifier(identifier),
            _ => {
                return Err(ParserError::Generic {
                    message: format!("expected identifier token got: {tok:?}"),
                });
            }
        };
        let type_annotation = if self
            .iter
            .bubble_next_if(|tok| matches!(tok, Token::Delimeter(Delimeter::Colon)))?
            .is_some()
        {
            tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
            Some(self.parse_concrete_kind(tok)?)
        } else {
            None
        };
        let lhs = DeclareLValue {
            identifier,
            type_annotation,
        };
        let ptok = self.iter.bubble_peek()?.ok_or(ParserError::Incomplete)?;
        if let Token::Delimeter(Delimeter::Equal) = ptok {
            let Some(Ok(Token::Delimeter(Delimeter::Equal))) = self.iter.next() else {
                unreachable!("validated by peek")
            };
            tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
            let rhs = Some(self.parse_expression(tok, Precedence::Assign)?);
            Ok(Expression::Operation(Box::new(Operation::Let { lhs, rhs })))
        } else {
            Ok(Expression::Operation(Box::new(Operation::Let {
                lhs,
                rhs: None,
            })))
        }
    }
    fn parse_nud_exit(&mut self) -> Result<Expression, ParserError<E>> {
        let ptok = self.iter.bubble_peek()?.ok_or(ParserError::Incomplete)?;
        if let Token::Delimeter(Delimeter::Semicolon) = ptok {
            Ok(Expression::Operation(Box::new(Operation::Exit(
                Expression::Concrete(Concrete::Integer(0)),
            ))))
        } else {
            let Some(Ok(tok)) = self.iter.next() else {
                unreachable!("validated by peek")
            };
            Ok(Expression::Operation(Box::new(Operation::Exit(
                self.parse_expression(tok, Precedence::Prefix)?,
            ))))
        }
    }
    pub fn parse_led(&mut self, lhs: Expression, tok: Token) -> Result<Expression, ParserError<E>> {
        match tok {
            Token::Delimeter(Delimeter::Plus) => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Term)?;
                Ok(Expression::Operation(Box::new(Operation::AddConcat {
                    lhs,
                    rhs,
                })))
            }
            Token::Delimeter(Delimeter::Minus) => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Term)?;
                Ok(Expression::Operation(Box::new(Operation::Subtract {
                    lhs,
                    rhs,
                })))
            }
            Token::Delimeter(Delimeter::Asterisk) => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Factor)?;
                Ok(Expression::Operation(Box::new(Operation::Multiply {
                    lhs,
                    rhs,
                })))
            }
            Token::Delimeter(Delimeter::ForwardSlash) => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Factor)?;
                Ok(Expression::Operation(Box::new(Operation::Divide {
                    lhs,
                    rhs,
                })))
            }
            Token::Delimeter(Delimeter::Percent) => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Factor)?;
                Ok(Expression::Operation(Box::new(Operation::Modulo {
                    lhs,
                    rhs,
                })))
            }
            Token::Delimeter(Delimeter::Colon) => self.parse_led_type(lhs),
            Token::Delimeter(Delimeter::Equal) => self.parse_led_assign(lhs),
            _ => Err(ParserError::Generic {
                message: format!("unexpected token: {tok:?}"),
            }),
        }
    }
    pub fn parse_mutable_lvalue(
        &mut self,
        mut expr: Expression,
    ) -> Result<MutableLValue, ParserError<E>> {
        let mut indices = Vec::new();
        while let Expression::Operation(op) = expr {
            if let Operation::Index { lhs, rhs } = *op {
                indices.push(rhs);
                expr = lhs;
            } else {
                return Err(ParserError::Generic {
                    message: format!("invalid assignment target: {op:?}"),
                });
            }
        }
        indices.reverse();
        match expr {
            Expression::Identifier(identifier) => Ok(MutableLValue {
                identifier,
                type_annotation: None,
                indices,
            }),
            _ => Err(ParserError::Generic {
                message: format!("invalid assignment target: {expr:?}"),
            }),
        }
    }
    pub fn parse_concrete_kind(&mut self, tok: Token) -> Result<ConcreteKind, ParserError<E>> {
        match tok {
            Token::Reserved(ReservedLexeme::Boolean) => Ok(ConcreteKind::Boolean),
            Token::Reserved(ReservedLexeme::Integer) => Ok(ConcreteKind::Integer),
            Token::Reserved(ReservedLexeme::Float) => Ok(ConcreteKind::Float),
            Token::Reserved(ReservedLexeme::String) => Ok(ConcreteKind::String),
            Token::Reserved(ReservedLexeme::List) => Ok(ConcreteKind::List),
            Token::Reserved(ReservedLexeme::Map) => Ok(ConcreteKind::Map),
            Token::Reserved(ReservedLexeme::Option) => Ok(ConcreteKind::Option),
            Token::Reserved(ReservedLexeme::Result) => Ok(ConcreteKind::Result),
            _ => Err(ParserError::Generic {
                message: format!("unexpected token: {tok:?}"),
            }),
        }
    }
    pub fn parse_led_type(&mut self, left: Expression) -> Result<Expression, ParserError<E>> {
        let mut lhs = self.parse_mutable_lvalue(left)?;

        let mut tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
        lhs.type_annotation = Some(self.parse_concrete_kind(tok)?);

        tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
        let Token::Delimeter(Delimeter::Equal) = tok else {
            return Err(ParserError::Generic {
                message: format!("expected = got {tok:?}"),
            });
        };

        tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
        let rhs = self.parse_expression(tok, Precedence::Assign)?;
        Ok(Expression::Operation(Box::new(Operation::Assign {
            lhs,
            rhs,
        })))
    }
    pub fn parse_led_assign(&mut self, left: Expression) -> Result<Expression, ParserError<E>> {
        let lhs = self.parse_mutable_lvalue(left)?;
        let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
        if let Token::Reserved(ReservedLexeme::Void) = tok {
            if !lhs.indices.is_empty() || lhs.type_annotation.is_some() {
                return Err(ParserError::Generic {
                    message: String::from("invalid lvalue for void assignment"),
                });
            }
            return Ok(Expression::Operation(Box::new(Operation::AssignVoid {
                lhs: lhs.identifier,
            })));
        }
        let rhs = self.parse_expression(tok, Precedence::Assign)?;
        Ok(Expression::Operation(Box::new(Operation::Assign {
            lhs,
            rhs,
        })))
    }
}

impl From<&Token> for Precedence {
    fn from(token: &Token) -> Self {
        match token {
            Token::Delimeter(delimeter) => match delimeter {
                Delimeter::Plus | Delimeter::Minus => Precedence::Term,
                Delimeter::Asterisk | Delimeter::ForwardSlash | Delimeter::Percent => {
                    Precedence::Factor
                }
                Delimeter::Equal | Delimeter::Colon => Precedence::Assign,
                _ => Precedence::None,
            },
            Token::Reserved(_)
            | Token::Float(_)
            | Token::Integer(_)
            | Token::LiteralString(_)
            | Token::Unreserved(_) => Precedence::None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assign,
    Comparison,
    Term,   // +, -
    Factor, // *, /, %
    Prefix, // -
}
impl Precedence {
    pub fn is_left_associative(&self) -> bool {
        match self {
            Precedence::Assign => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
pub enum ParserError<E> {
    Source(E),
    Generic { message: String },
    Incomplete,
}

impl<I, E> Iterator for Parser<I, E>
where
    I: ErrorBubbledNLookahead<2, Token, ParserError<E>>,
{
    type Item = Result<Expression, ParserError<E>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse_top_level_expression()
    }
}
