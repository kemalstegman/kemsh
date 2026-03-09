use std::iter::Peekable;

use crate::{
    executor::{EchoInstruction, Expression, Instruction, LetInstruction, VariableValue},
    lexer::{Token, TokenDelimeter, TokenKeyword, TokenNumber},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ParseError {
    message: String,
}

pub fn parse(
    mut token_stream: Peekable<impl Iterator<Item = Token>>,
) -> Result<Option<Vec<Instruction>>, ParseError> {
    let mut commands: Vec<Instruction> = Vec::new();
    while let Some(tok) = token_stream.next() {
        match tok {
            Token::Keyword(TokenKeyword::Let) => match token_stream.next() {
                None => return Ok(None),
                Some(Token::VariableName(var_name)) => match token_stream.next() {
                    None => return Ok(None),
                    Some(Token::Delimeter(TokenDelimeter::Equal)) => {
                        match parse_expression(&mut token_stream) {
                            Ok(None) => return Ok(None),
                            Ok(Some(e)) => commands.push(Instruction::Let(LetInstruction {
                                variable_name: var_name,
                                expression: Some(e),
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
            Token::Keyword(TokenKeyword::Echo) => match parse_expression(&mut token_stream) {
                Ok(None) => return Ok(None),
                Ok(Some(e)) => commands.push(Instruction::Echo(EchoInstruction { expression: e })),
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
    match token_stream.next() {
        None => Ok(None),
        Some(Token::Boolean(b)) => match token_stream.next() {
            Some(Token::Delimeter(TokenDelimeter::Semicolon)) => {
                Ok(Some(Expression::Value(VariableValue::LiteralBoolean(b))))
            }
            _ => todo!(),
        },
        Some(Token::Number(TokenNumber { string })) => match token_stream.next() {
            Some(Token::Delimeter(TokenDelimeter::Semicolon)) => match string.parse::<i64>() {
                Ok(n) => Ok(Some(Expression::Value(VariableValue::LiteralInteger(n)))),
                _ => todo!(),
            },
            _ => todo!(),
        },
        Some(Token::String(s)) => match token_stream.next() {
            Some(Token::Delimeter(TokenDelimeter::Semicolon)) => {
                Ok(Some(Expression::Value(VariableValue::LiteralString(s))))
            }
            _ => todo!(),
        },
        Some(Token::VariableName(v)) => match token_stream.next() {
            Some(Token::Delimeter(TokenDelimeter::Semicolon)) => Ok(Some(Expression::Variable(v))),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
