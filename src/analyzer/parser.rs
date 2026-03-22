use std::iter::Peekable;

use crate::abstract_syntax_tree::{EchoInstruction, Expression, LetInstruction};
use crate::analyzer::lexer::{TokenDelimeter, TokenKeyword};

pub use super::AnalyzerError;

use super::super::abstract_syntax_tree::Instruction;
use super::lexer::{LexError, Token};

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

pub struct Parser<I, E>
where
    I: IntoIterator<Item = Result<Token, AnalyzerError<E>>>,
{
    iter: Peekable<I::IntoIter>,
}

impl<I, E> Parser<I, E>
where
    I: IntoIterator<Item = Result<Token, AnalyzerError<E>>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.into_iter().peekable(),
        }
    }
    fn parse(&mut self) -> Option<Result<Instruction, AnalyzerError<E>>> {
        match self.iter.next()? {
            Err(e) => Some(Err(e)),
            Ok(tok) => Some(self.parse_instruction(tok)),
        }
    }
    fn peek_token(&mut self) -> Result<&Token, AnalyzerError<E>> {
        if let Some(Err(_)) = self.iter.peek() {
            return Err(self.iter.next().unwrap().unwrap_err());
        }
        match self.iter.peek() {
            None => Err(AnalyzerError::Incomplete),
            Some(Ok(tok)) => Ok(tok),
            Some(Err(_)) => unreachable!(),
        }
    }
    fn next_token(&mut self) -> Result<Token, AnalyzerError<E>> {
        self.iter.next().ok_or(AnalyzerError::Incomplete).flatten()
    }
    fn next_token_if(
        &mut self,
        func: impl FnOnce(&Token) -> bool,
    ) -> Result<Option<Token>, AnalyzerError<E>> {
        if func(self.peek_token()?) {
            self.iter.next().transpose()
        } else {
            Ok(None)
        }
    }
    fn next_token_if_eq(&mut self, tok: &Token) -> Result<Option<Token>, AnalyzerError<E>> {
        self.next_token_if(|peek_tok| peek_tok == tok)
    }
    fn parse_instruction(&mut self, first_token: Token) -> Result<Instruction, AnalyzerError<E>> {
        match first_token {
            Token::Keyword(TokenKeyword::Let) => match self.next_token()? {
                Token::VariableName(variable_name) => match self.next_token()? {
                    Token::Delimeter(TokenDelimeter::Equal) => {
                        Ok(Instruction::Let(LetInstruction {
                            variable_name,
                            variable_kind: None,
                            expression: Some(self.parse_expression()?),
                        }))
                    }
                    _ => Err(AnalyzerError::Parser(ParseError {
                        message: String::from("expected ="),
                    })),
                },
                _ => Err(AnalyzerError::Parser(ParseError {
                    message: String::from("expected variable name"),
                })),
            },
            Token::Keyword(TokenKeyword::Echo) => Ok(Instruction::Echo(EchoInstruction {
                expressions: vec![self.parse_expression()?],
            })),
            tok => Err(AnalyzerError::Parser(ParseError {
                message: format!("Token unimplemented: {tok:?}"),
            })),
        }
    }
    fn parse_expression(&mut self) -> Result<Expression, AnalyzerError<E>> {
        todo!()
    }
}

impl<I, E> Iterator for Parser<I, E>
where
    I: IntoIterator<Item = Result<Token, AnalyzerError<E>>>,
{
    type Item = Result<Instruction, AnalyzerError<E>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}
