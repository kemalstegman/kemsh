use std::iter::Peekable;

use crate::{
    executor::{
        expression::Expression,
        instruction::{EchoInstruction, Instruction, LetInstruction, SetInstruction},
        variables::VariableValue,
    },
    lexer::{FatalLexError, LexError, Lexer, Token, TokenDelimeter, TokenKeyword},
};

#[derive(Debug)]
pub enum ParseError {
    Char(()),
    Token(FatalLexError),
    Parse(FatalParseError),
    Incomplete,
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> Self {
        match err {
            LexError::Char(err) => ParseError::Char(err),
            LexError::Lex(err) => ParseError::Token(err),
            LexError::Incomplete => ParseError::Incomplete,
        }
    }
}

#[derive(Debug)]
pub struct FatalParseError {
    message: String,
}

pub struct Parser<I>
where
    I: Iterator<Item = Result<char, ()>>,
{
    iter: Peekable<Lexer<I>>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Result<char, ()>>,
{
    pub fn new(lexer: Lexer<I>) -> Self {
        Self {
            iter: lexer.peekable(),
        }
    }
    fn parse(&mut self) -> Option<Result<Instruction, ParseError>> {
        todo!()
    }
}

impl<I> Iterator for Parser<I>
where
    I: Iterator<Item = Result<char, ()>>,
{
    type Item = Result<Instruction, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

// #[derive(Debug)]
// pub struct ParseError {
//     message: String,
// }

// pub struct Parser<I: Iterator<Item = Token>> {
//     iter: Peekable<I>,
// }

// impl<I: Iterator<Item = Token>> Parser<I> {
//     pub fn new(iter: I) -> Self {
//         Self {
//             iter: iter.peekable(),
//         }
//     }
//     pub fn parse_instruction(&mut self) -> Option<Result<Option<Instruction>, ParseError>> {
//         match self.iter.next()? {
//             Token::Keyword(TokenKeyword::Let) => match self.iter.next() {
//                 None => return Some(Ok(None)),
//                 Some(Token::VariableName(var_name)) => match self.iter.next() {
//                     None => return Some(Ok(None)),
//                     Some(Token::Delimeter(TokenDelimeter::Equal)) => {
//                         match parse_expression(&mut self.iter) {
//                             Ok(None) => return Some(Ok(None)),
//                             Ok(Some(e)) => {
//                                 return Some(Ok(Some(Instruction::Let(LetInstruction {
//                                     variable_name: var_name,
//                                     variable_kind: None,
//                                     expression: Some(e),
//                                 }))));
//                             }
//                             Err(()) => {
//                                 return Some(Err(ParseError {
//                                     message: "expected expression".to_string(),
//                                 }));
//                             }
//                         }
//                     }
//                     _ => {
//                         return Some(Err(ParseError {
//                             message: "expected =".to_string(),
//                         }));
//                     }
//                 },
//                 _ => {
//                     return Some(Err(ParseError {
//                         message: "expected variable name".to_string(),
//                     }));
//                 }
//             },
//             Token::Keyword(TokenKeyword::Echo) => match parse_expression(&mut self.iter) {
//                 Ok(None) => return Some(Ok(None)),
//                 Ok(Some(e)) => Some(Ok(Some(Instruction::Echo(EchoInstruction {
//                     expressions: vec![e],
//                 })))),
//                 Err(()) => {
//                     return Some(Err(ParseError {
//                         message: "expected expression".to_string(),
//                     }));
//                 }
//             },
//             // Token::Keyword(TokenKeyword::Run) => {}
//             Token::VariableName(variable_name) => match self.iter.next() {
//                 None => return Some(Ok(None)),
//                 Some(Token::Delimeter(TokenDelimeter::Equal)) => {
//                     match parse_expression(&mut self.iter) {
//                         Ok(None) => return Some(Ok(None)),
//                         Ok(Some(e)) => Some(Ok(Some(Instruction::Set(SetInstruction {
//                             variable_name: variable_name,
//                             expression: e,
//                         })))),
//                         Err(()) => {
//                             return Some(Err(ParseError {
//                                 message: "expected expression".to_string(),
//                             }));
//                         }
//                     }
//                 }
//                 _ => {
//                     return Some(Err(ParseError {
//                         message: "expected =".to_string(),
//                     }));
//                 }
//             },
//             tok => {
//                 return Some(Err(ParseError {
//                     message: format!("Token unimplemented: {tok:?}"),
//                 }));
//             }
//         }
//     }
// }

// impl<I: Iterator<Item = Token>> Iterator for Parser<I> {
//     type Item = Result<Option<Instruction>, ParseError>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.parse_instruction()
//     }
// }

// fn parse_expression(
//     token_stream: &mut Peekable<impl Iterator<Item = Token>>,
// ) -> Result<Option<Expression>, ()> {
//     match token_stream.next() {
//         None => Ok(None),
//         Some(Token::Boolean(b)) => match token_stream.next() {
//             Some(Token::Delimeter(TokenDelimeter::Semicolon)) => {
//                 Ok(Some(Expression::Value(VariableValue::LiteralBoolean(b))))
//             }
//             _ => todo!(),
//         },
//         Some(Token::Number(n)) => match token_stream.next() {
//             Some(Token::Delimeter(TokenDelimeter::Semicolon)) => {
//                 Ok(Some(Expression::Value(VariableValue::LiteralInteger(n))))
//             }
//             _ => todo!(),
//         },
//         Some(Token::String(s)) => match token_stream.next() {
//             Some(Token::Delimeter(TokenDelimeter::Semicolon)) => {
//                 Ok(Some(Expression::Value(VariableValue::LiteralString(s))))
//             }
//             _ => todo!(),
//         },
//         Some(Token::VariableName(v)) => match token_stream.next() {
//             Some(Token::Delimeter(TokenDelimeter::Semicolon)) => Ok(Some(Expression::Variable(v))),
//             _ => todo!(),
//         },
//         _ => todo!(),
//     }
// }

// pub struct TokenBridge<I: Iterator<Item = Result<Option<Token>, LexError>>> {
//     iter: I,
//     error: Option<Result<(), LexError>>,
// }

// impl<I: Iterator<Item = Result<Option<Token>, LexError>>> Iterator for TokenBridge<I> {
//     type Item = Token;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.error.is_some() {
//             return None;
//         }
//         match self.iter.next() {
//             None => None,
//             Some(Ok(Some(tok))) => {
//                 // println!("{tok:?}");
//                 Some(tok)
//             }
//             Some(Ok(None)) => {
//                 self.error = Some(Ok(()));
//                 None
//             }
//             Some(Err(e)) => {
//                 self.error = Some(Err(e));
//                 None
//             }
//         }
//     }
// }

// impl<I: Iterator<Item = Result<Option<Token>, LexError>>> TokenBridge<I> {
//     pub fn new(iter: I) -> Self {
//         Self { iter, error: None }
//     }
//     pub fn take_error(&mut self) -> Option<Result<(), LexError>> {
//         self.error.take()
//     }
// }

// pub struct InstructionBridge<I: Iterator<Item = Result<Option<Instruction>, ParseError>>> {
//     iter: I,
//     error: Option<Result<(), ParseError>>,
// }

// impl<I: Iterator<Item = Result<Option<Instruction>, ParseError>>> Iterator
//     for InstructionBridge<I>
// {
//     type Item = Instruction;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.error.is_some() {
//             return None;
//         }
//         match self.iter.next() {
//             None => None,
//             Some(Ok(Some(instruction))) => Some(instruction),
//             Some(Ok(None)) => {
//                 self.error = Some(Ok(()));
//                 None
//             }
//             Some(Err(e)) => {
//                 self.error = Some(Err(e));
//                 None
//             }
//         }
//     }
// }

// impl<I: Iterator<Item = Result<Option<Instruction>, ParseError>>> InstructionBridge<I> {
//     pub fn new(iter: I) -> Self {
//         Self { iter, error: None }
//     }
//     pub fn take_error(&mut self) -> Option<Result<(), ParseError>> {
//         self.error.take()
//     }
// }
