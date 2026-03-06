use std::iter::Peekable;

use crate::lexer::{Token, TokenDelimeter, TokenKeyword};

pub enum ShellCommand {
    Let(LetCommand),
    Run(RunCommand),
    Print(Expression),
}

pub enum Operation {
    Add { lhs: Expression, rhs: Expression },
    Subtract { lhs: Expression, rhs: Expression },
    Multiply { lhs: Expression, rhs: Expression },
    Divide { lhs: Expression, rhs: Expression },
}

pub enum Expression {
    Operation(Box<Operation>),
    // Block(Block),
    Variable(String),
    Literal(Literal),
}

pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
}

pub struct LetCommand {
    variable: String,
    value: Expression,
}

pub enum RunCommand {
    OneString(String),
    StringArray(Vec<String>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParseError {
    message: String,
}

pub fn parse(
    mut token_stream: Peekable<impl Iterator<Item = Token>>,
) -> Result<Option<Vec<ShellCommand>>, ParseError> {
    let mut commands: Vec<ShellCommand> = Vec::new();
    while let Some(tok) = token_stream.next() {
        match tok {
            Token::Keyword(TokenKeyword::Let) => match token_stream.next() {
                None => return Ok(None),
                Some(Token::VariableName(var_name)) => match token_stream.next() {
                    None => return Ok(None),
                    Some(Token::Delimeter(TokenDelimeter::Equal)) => {
                        match parse_expression(&mut token_stream) {
                            Ok(None) => return Ok(None),
                            Ok(Some(e)) => commands.push(ShellCommand::Let(LetCommand {
                                variable: var_name,
                                value: e,
                            })),
                            Err(()) => {
                                return Err(ParseError {
                                    message: "expected expression".to_string(),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError {
                            message: "expected =".to_string(),
                        });
                    }
                },
                _ => {
                    return Err(ParseError {
                        message: "expected variable name".to_string(),
                    });
                }
            },
            Token::Keyword(TokenKeyword::Print) => match parse_expression(&mut token_stream) {
                Ok(None) => return Ok(None),
                Ok(Some(e)) => commands.push(ShellCommand::Print(e)),
                Err(()) => {
                    return Err(ParseError {
                        message: "expected expression".to_string(),
                    });
                }
            },
            // Token::Keyword(TokenKeyword::Run) => {}
            _ => {
                return Err(ParseError {
                    message: format!("Token unimplemented: {tok:?}"),
                });
            }
        }
    }
    Ok(Some(commands))
}

fn parse_expression(
    token_stream: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Option<Expression>, ()> {
    todo!()
}
