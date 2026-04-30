//!

use std::{marker::PhantomData, rc::Rc};

use crate::{
    abstract_lookahead::ErrorBubbledNLookahead,
    ast::{
        BraceBlock, CompoundLiteral, DeclareLValue, Expression, Identifier, Literal, MutableLValue,
        Operation,
    },
    executor::concrete::ConcreteKind,
    syntax::lexer::token::Token,
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
                    Ok(Token::DELIM_SEMICOLON) => Some(Ok(expression)),
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
            Token::Float(n) => Ok(Expression::Literal(Literal::Float(n))),
            Token::Integer(n) => Ok(Expression::Literal(Literal::Integer(n))),
            Token::RawString(s) => Ok(Expression::Literal(Literal::RawString(s))),
            Token::LEX_TRUE => Ok(Expression::Literal(Literal::Boolean(true))),
            Token::LEX_FALSE => Ok(Expression::Literal(Literal::Boolean(false))),
            Token::UnreservedLexeme(i) => Ok(Expression::Identifier(Identifier(i))),
            Token::LEX_LET => Ok(self.parse_nud_let()?),
            Token::LEX_EXIT => Ok(self.parse_nud_exit()?),
            Token::LEX_CD => {
                let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                Ok(Expression::Operation(Box::new(Operation::ChangeDirectory(
                    self.parse_expression(tok, Precedence::Prefix)?,
                ))))
            }
            Token::LEX_RUN => {
                let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                Ok(Expression::Operation(Box::new(Operation::Run(
                    self.parse_expression(tok, Precedence::Prefix)?,
                ))))
            }
            Token::LEX_SPAWN => {
                let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                Ok(Expression::Operation(Box::new(Operation::Spawn(
                    self.parse_expression(tok, Precedence::Prefix)?,
                ))))
            }
            Token::LEX_ECHO => {
                let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                Ok(Expression::Operation(Box::new(Operation::Echo(
                    self.parse_expression(tok, Precedence::Prefix)?,
                ))))
            }
            Token::LEX_ECHON => {
                let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                Ok(Expression::Operation(Box::new(Operation::Echon(
                    self.parse_expression(tok, Precedence::Prefix)?,
                ))))
            }
            Token::DELIM_OPENBRACKET => {
                let mut expressions = Vec::new();
                if self
                    .iter
                    .bubble_next_if(|tok| matches!(tok, Token::DELIM_CLOSEBRACKET))?
                    .is_some()
                {
                    return Ok(Expression::CompoundLiteral(CompoundLiteral::List(
                        expressions,
                    )));
                }
                loop {
                    let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                    expressions.push(self.parse_expression(tok, Precedence::None)?);
                    if self
                        .iter
                        .bubble_next_if(|tok| matches!(tok, Token::DELIM_COMMA))?
                        .is_none()
                    {
                        break;
                    }
                }
                match self.iter.next().ok_or(ParserError::Incomplete).flatten()? {
                    Token::DELIM_CLOSEBRACKET => (),
                    tok => {
                        return Err(ParserError::Generic {
                            message: format!("expected ] got {tok:?}"),
                        });
                    }
                }
                Ok(Expression::CompoundLiteral(CompoundLiteral::List(
                    expressions,
                )))
            }
            Token::DELIM_OPENBRACE => {
                let mut expressions = Vec::new();
                let mut evaluate_to_tail_expression = false;
                loop {
                    let mut tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                    if let Token::DELIM_CLOSEBRACE = tok {
                        break;
                    }
                    expressions.push(self.parse_expression(tok, Precedence::None)?);
                    tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                    match tok {
                        Token::DELIM_SEMICOLON => (),
                        Token::DELIM_CLOSEBRACE => {
                            evaluate_to_tail_expression = true;
                            break;
                        }
                        tok => {
                            return Err(ParserError::Generic {
                                message: format!("expected ; or }} got {tok:?}"),
                            });
                        }
                    }
                }
                if expressions.is_empty() {
                    return Err(ParserError::Generic {
                        message: format!("expected at least one expression inside brace block"),
                    });
                }
                Ok(Expression::BraceBlock(BraceBlock {
                    expressions,
                    evaluate_to_tail_expression,
                }))
            }
            Token::LEX_FN => {
                let mut tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                match tok {
                    Token::DELIM_OPENPARENTHESIS => (),
                    _ => {
                        return Err(ParserError::Generic {
                            message: format!("expected ) got {tok:?}"),
                        });
                    }
                }
                tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let mut arguments = Vec::new();
                if let Token::UnreservedLexeme(i) = tok {
                    arguments.push((i, None));
                    tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                    loop {
                        match tok {
                            Token::DELIM_CLOSEPARENTHESIS => break,
                            Token::DELIM_COMMA => {
                                tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                                match tok {
                                    Token::UnreservedLexeme(i) => arguments.push((i, None)),
                                    tok => {
                                        return Err(ParserError::Generic {
                                            message: format!("expected identifier got {tok:?}"),
                                        });
                                    }
                                }
                                tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                            }
                            tok => {
                                return Err(ParserError::Generic {
                                    message: format!("expected ) or , got {tok:?}"),
                                });
                            }
                        }
                    }
                } else if let Token::DELIM_CLOSEPARENTHESIS = tok {
                    ()
                } else {
                    return Err(ParserError::Generic {
                        message: format!("expected identifier or ) got {tok:?}"),
                    });
                }
                tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                match tok {
                    Token::DELIM_OPENBRACE => (),
                    _ => {
                        return Err(ParserError::Generic {
                            message: format!("expected {{ got {tok:?}"),
                        });
                    }
                }
                let mut expressions = Vec::new();
                let mut tail = false;
                loop {
                    tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                    if let Token::DELIM_CLOSEBRACE = tok {
                        break;
                    }
                    expressions.push(self.parse_expression(tok, Precedence::None)?);
                    tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                    match tok {
                        Token::DELIM_SEMICOLON => (),
                        Token::DELIM_CLOSEBRACE => {
                            tail = true;
                            break;
                        }
                        tok => {
                            return Err(ParserError::Generic {
                                message: format!("expected ; or }} got {tok:?}"),
                            });
                        }
                    }
                }
                Ok(Expression::CompoundLiteral(CompoundLiteral::Function(
                    Rc::from(expressions),
                    Rc::from(arguments),
                    tail,
                )))
            }
            // map
            // option
            // result
            // control flow
            // function
            _ => Err(ParserError::Generic {
                message: format!("unexpected token: {tok:?}"),
            }),
        }
    }
    pub fn parse_nud_let(&mut self) -> Result<Expression, ParserError<E>> {
        let mut tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
        let identifier = match tok {
            Token::UnreservedLexeme(i) => Identifier(i),
            _ => {
                return Err(ParserError::Generic {
                    message: format!("expected identifier token got: {tok:?}"),
                });
            }
        };
        let type_annotation = if self
            .iter
            .bubble_next_if(|tok| matches!(tok, Token::DELIM_COLON))?
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
        if let Token::DELIM_EQUAL = ptok {
            let Some(Ok(Token::DELIM_EQUAL)) = self.iter.next() else {
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
        if let Token::DELIM_SEMICOLON = ptok {
            Ok(Expression::Operation(Box::new(Operation::Exit(
                Expression::Literal(Literal::Integer(0)),
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
            Token::DELIM_PLUS => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Term)?;
                Ok(Expression::Operation(Box::new(Operation::AddConcat {
                    lhs,
                    rhs,
                })))
            }
            Token::DELIM_MINUS => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Term)?;
                Ok(Expression::Operation(Box::new(Operation::Subtract {
                    lhs,
                    rhs,
                })))
            }
            Token::DELIM_ASTERISK => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Factor)?;
                Ok(Expression::Operation(Box::new(Operation::Multiply {
                    lhs,
                    rhs,
                })))
            }
            Token::DELIM_FORWARDSLASH => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Factor)?;
                Ok(Expression::Operation(Box::new(Operation::Divide {
                    lhs,
                    rhs,
                })))
            }
            Token::DELIM_PERCENT => {
                let next_tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                let rhs = self.parse_expression(next_tok, Precedence::Factor)?;
                Ok(Expression::Operation(Box::new(Operation::Modulo {
                    lhs,
                    rhs,
                })))
            }
            Token::DELIM_COLON => self.parse_led_type(lhs),
            Token::DELIM_EQUAL => self.parse_led_assign(lhs),
            Token::DELIM_OPENPARENTHESIS => {
                let mut arguments = Vec::new();
                if self
                    .iter
                    .bubble_next_if(|tok| matches!(tok, Token::DELIM_CLOSEPARENTHESIS))?
                    .is_none()
                {
                    loop {
                        let tok = self.iter.next().ok_or(ParserError::Incomplete).flatten()?;
                        arguments.push(self.parse_expression(tok, Precedence::None)?);
                        if self
                            .iter
                            .bubble_next_if(|tok| matches!(tok, Token::DELIM_COMMA))?
                            .is_none()
                        {
                            break;
                        }
                    }
                    match self.iter.next().ok_or(ParserError::Incomplete).flatten()? {
                        Token::DELIM_CLOSEPARENTHESIS => (),
                        tok => {
                            return Err(ParserError::Generic {
                                message: format!("expected ) got {tok:?}"),
                            });
                        }
                    }
                }
                Ok(Expression::Operation(Box::new(Operation::Call(
                    lhs, arguments,
                ))))
            }
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
            Token::LEX_BOOLEAN => Ok(ConcreteKind::Boolean),
            Token::LEX_INTEGER => Ok(ConcreteKind::Integer),
            Token::LEX_FLOAT => Ok(ConcreteKind::Float),
            Token::LEX_STRING => Ok(ConcreteKind::String),
            Token::LEX_LIST => Ok(ConcreteKind::List),
            Token::LEX_MAP => Ok(ConcreteKind::Map),
            Token::LEX_OPTION => Ok(ConcreteKind::Option),
            Token::LEX_RESULT => Ok(ConcreteKind::Result),
            Token::LEX_FUNCTION => Ok(ConcreteKind::Function),
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
        let Token::DELIM_EQUAL = tok else {
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
        if let Token::LEX_VOID = tok {
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
            Token::DELIM_PLUS | Token::DELIM_MINUS => Precedence::Term,
            Token::DELIM_ASTERISK | Token::DELIM_FORWARDSLASH | Token::DELIM_PERCENT => {
                Precedence::Factor
            }
            Token::DELIM_EQUAL | Token::DELIM_COLON => Precedence::Assign,
            Token::DELIM_OPENPARENTHESIS => Precedence::Call,
            _ => Precedence::None,
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
    Call,
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
