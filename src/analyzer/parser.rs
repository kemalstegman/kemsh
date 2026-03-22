use std::iter::Peekable;

use crate::abstract_syntax_tree::{
    ChangeDirectoryInstruction, EchoInstruction, ExitInstruction, Expression, Instruction,
    LetInstruction, SetInstruction, VariableKind, VariableValue,
};
use crate::analyzer::AnalyzerError;
use crate::analyzer::lexer::token::{Token, TokenDelimeter, TokenKeyword, TokenVariableKind};

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
                        let expression = self.parse_expression()?;
                        match self.next_token()? {
                            Token::Delimeter(TokenDelimeter::Semicolon) => {
                                Ok(Instruction::Let(LetInstruction {
                                    variable_name,
                                    variable_kind: None,
                                    expression: Some(expression),
                                }))
                            }
                            _ => Err(AnalyzerError::Parser(ParseError {
                                message: String::from("expected ;"),
                            })),
                        }
                    }
                    Token::Delimeter(TokenDelimeter::Colon) => {
                        let variable_kind = self.parse_variable_kind()?;
                        match self.next_token()? {
                            Token::Delimeter(TokenDelimeter::Equal) => {
                                let expression = self.parse_expression()?;
                                match self.next_token()? {
                                    Token::Delimeter(TokenDelimeter::Semicolon) => {
                                        Ok(Instruction::Let(LetInstruction {
                                            variable_name,
                                            variable_kind: Some(variable_kind),
                                            expression: Some(expression),
                                        }))
                                    }
                                    _ => Err(AnalyzerError::Parser(ParseError {
                                        message: String::from("expected ;"),
                                    })),
                                }
                            }
                            Token::Delimeter(TokenDelimeter::Semicolon) => {
                                Ok(Instruction::Let(LetInstruction {
                                    variable_name,
                                    variable_kind: Some(variable_kind),
                                    expression: None,
                                }))
                            }
                            _ => Err(AnalyzerError::Parser(ParseError {
                                message: String::from("expected = or ;"),
                            })),
                        }
                    }
                    Token::Delimeter(TokenDelimeter::Semicolon) => {
                        Ok(Instruction::Let(LetInstruction {
                            variable_name,
                            variable_kind: None,
                            expression: None,
                        }))
                    }
                    _ => Err(AnalyzerError::Parser(ParseError {
                        message: String::from("expected = or : or ;"),
                    })),
                },
                _ => Err(AnalyzerError::Parser(ParseError {
                    message: String::from("expected variable name"),
                })),
            },
            Token::VariableName(variable_name) => match self.next_token()? {
                Token::Delimeter(TokenDelimeter::Equal) => {
                    let expression = self.parse_expression()?;
                    match self.next_token()? {
                        Token::Delimeter(TokenDelimeter::Semicolon) => {
                            Ok(Instruction::Set(SetInstruction {
                                variable_name,
                                expression,
                            }))
                        }
                        _ => Err(AnalyzerError::Parser(ParseError {
                            message: String::from("expected ;"),
                        })),
                    }
                }
                _ => Err(AnalyzerError::Parser(ParseError {
                    message: String::from("expected ="),
                })),
            },
            Token::Keyword(TokenKeyword::Echo) => {
                let expression = self.parse_expression()?;
                match self.next_token()? {
                    Token::Delimeter(TokenDelimeter::Semicolon) => {
                        Ok(Instruction::Echo(EchoInstruction {
                            expressions: vec![expression],
                        }))
                    }
                    _ => Err(AnalyzerError::Parser(ParseError {
                        message: String::from("expected ;"),
                    })),
                }
            }
            Token::Keyword(TokenKeyword::Cd) => {
                let expression = self.parse_expression()?;
                match self.next_token()? {
                    Token::Delimeter(TokenDelimeter::Semicolon) => {
                        Ok(Instruction::ChangeDirectory(ChangeDirectoryInstruction {
                            expression,
                        }))
                    }
                    _ => Err(AnalyzerError::Parser(ParseError {
                        message: String::from("expected ;"),
                    })),
                }
            }
            Token::Keyword(TokenKeyword::Exit) => {
                if self
                    .next_token_if_eq(&Token::Delimeter(TokenDelimeter::Semicolon))?
                    .is_some()
                {
                    Ok(Instruction::Exit(ExitInstruction {
                        expression: Expression::Value(VariableValue::LiteralInteger(0)),
                    }))
                } else {
                    let expression = self.parse_expression()?;
                    match self.next_token()? {
                        Token::Delimeter(TokenDelimeter::Semicolon) => {
                            Ok(Instruction::Exit(ExitInstruction { expression }))
                        }
                        _ => Err(AnalyzerError::Parser(ParseError {
                            message: String::from("expected ;"),
                        })),
                    }
                }
            }
            tok => Err(AnalyzerError::Parser(ParseError {
                message: format!("Token unimplemented: {tok:?}"),
            })),
        }
    }
    fn parse_expression(&mut self) -> Result<Expression, AnalyzerError<E>> {
        match self.next_token()? {
            Token::Boolean(boolean) => {
                Ok(Expression::Value(VariableValue::LiteralBoolean(boolean)))
            }
            Token::Integer(number) => Ok(Expression::Value(VariableValue::LiteralInteger(number))),
            Token::String(string) => Ok(Expression::Value(VariableValue::LiteralString(string))),
            Token::VariableName(variable_name) => Ok(Expression::Variable(variable_name)),
            _ => todo!(),
        }
    }
    fn parse_variable_kind(&mut self) -> Result<VariableKind, AnalyzerError<E>> {
        match self.next_token()? {
            Token::VariableKind(kind) => match kind {
                TokenVariableKind::Boolean => Ok(VariableKind::LiteralBoolean),
                TokenVariableKind::Integer => Ok(VariableKind::LiteralInteger),
                TokenVariableKind::String => Ok(VariableKind::LiteralString),
            },
            _ => todo!(),
        }
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
