use std::iter::FusedIterator;

use crate::{
    ast::{AST, Literal, Operation},
    lexer::{
        Delimiter,
        Lexeme::{self, Identifier},
        Number, Token,
    },
    npeekable::NPeekable,
};

type PToken<'a> = Token<Lexeme<'a>, Number<'a>, Delimiter, &'a str, (), &'a str>;

enum NullDenotation<'a> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(&'a str),
    IDENTIFIER(&'a str),
    Negate,
}

impl<'a> TryFrom<PToken<'a>> for NullDenotation<'a> {
    type Error = ();
    fn try_from(token: PToken<'a>) -> Result<Self, Self::Error> {
        match token {
            Token::Lexeme(Identifier(identifier)) => Ok(NullDenotation::IDENTIFIER(identifier)),
            Token::Lexeme(Lexeme::LexTrue) => Ok(NullDenotation::BOOL(true)),
            Token::Lexeme(Lexeme::LexFalse) => Ok(NullDenotation::BOOL(false)),
            Token::Number(Number::Binary(number_string)) => {
                match u64::from_str_radix(number_string, 2) {
                    Ok(number) => Ok(NullDenotation::U64(number)),
                    Err(_e) => todo!(),
                }
            }
            Token::Number(Number::Hex(number_string)) => {
                match u64::from_str_radix(number_string, 16) {
                    Ok(number) => Ok(NullDenotation::U64(number)),
                    Err(_e) => todo!(),
                }
            }
            Token::Number(Number::Unsigned(number_string)) => {
                match u64::from_str_radix(number_string, 10) {
                    Ok(number) => Ok(NullDenotation::U64(number)),
                    Err(_e) => todo!(),
                }
            }
            Token::Number(Number::Signed(number_string))
            | Token::Number(Number::Integer(number_string)) => {
                match i64::from_str_radix(number_string, 10) {
                    Ok(number) => Ok(NullDenotation::I64(number)),
                    Err(_e) => todo!(),
                }
            }
            Token::Number(Number::Float(number_string)) => match number_string.parse::<f64>() {
                Ok(number) => Ok(NullDenotation::F64(number)),
                Err(_e) => todo!(),
            },
            Token::RawString(string) => Ok(NullDenotation::STRING(string)),
            Token::Lexeme(Lexeme::LexUnit) => todo!(),
            Token::Lexeme(Lexeme::LexOk) => todo!(),
            Token::Lexeme(Lexeme::LexErr) => todo!(),
            Token::Lexeme(Lexeme::LexSome) => todo!(),
            Token::Lexeme(Lexeme::LexNone) => todo!(),
            Token::Lexeme(Lexeme::LexCd) => todo!(),
            Token::Lexeme(Lexeme::LexProcess) => todo!(),
            Token::Lexeme(Lexeme::LexRun) => todo!(),
            Token::Lexeme(Lexeme::LexSpawn) => todo!(),
            Token::Lexeme(Lexeme::LexWait) => todo!(),
            Token::Lexeme(Lexeme::LexDrop) => todo!(),
            // let
            Token::Lexeme(Lexeme::LexLet) => todo!(),
            // control flow
            Token::Lexeme(Lexeme::LexLoop) => todo!(),
            Token::Lexeme(Lexeme::LexFor) => todo!(),
            Token::Lexeme(Lexeme::LexWhile) => todo!(),
            Token::Lexeme(Lexeme::LexIf) => todo!(),
            Token::Lexeme(Lexeme::LexFn) => todo!(),
            // unexpected type lexemes
            Token::Lexeme(Lexeme::LexI64)
            | Token::Lexeme(Lexeme::LexU64)
            | Token::Lexeme(Lexeme::LexBool)
            | Token::Lexeme(Lexeme::LexString)
            | Token::Lexeme(Lexeme::LexList)
            | Token::Lexeme(Lexeme::LexMap)
            | Token::Lexeme(Lexeme::LexOption)
            | Token::Lexeme(Lexeme::LexResult)
            | Token::Lexeme(Lexeme::LexJcmd)
            | Token::Lexeme(Lexeme::LexJhandle)
            | Token::Lexeme(Lexeme::LexJexit)
            | Token::Lexeme(Lexeme::LexFhandle) => todo!(),
            Token::Delimiter(Delimiter::OpenBrace) => todo!(),
            Token::Delimiter(Delimiter::OpenSquareBracket) => todo!(),
            Token::Delimiter(Delimiter::OpenParenthesis) => todo!(),
            Token::Delimiter(Delimiter::HyphenMinus) => Ok(NullDenotation::Negate),
            Token::Delimiter(Delimiter::Plus) => todo!(),
            Token::Delimiter(Delimiter::OpenBraceBracket) => todo!(),
            // unexpected delimiters
            Token::Delimiter(Delimiter::OpenAngleBracket)
            | Token::Delimiter(Delimiter::CloseAngleBracket)
            | Token::Delimiter(Delimiter::CloseSquareBracket)
            | Token::Delimiter(Delimiter::CloseParenthesis)
            | Token::Delimiter(Delimiter::CloseBrace)
            | Token::Delimiter(Delimiter::CloseBracketBrace)
            | Token::Delimiter(Delimiter::Comma)
            | Token::Delimiter(Delimiter::Period)
            | Token::Delimiter(Delimiter::Semicolon)
            | Token::Delimiter(Delimiter::Colon)
            | Token::Delimiter(Delimiter::Equal)
            | Token::Delimiter(Delimiter::ForwardSlash)
            | Token::Delimiter(Delimiter::BackSlash)
            | Token::Delimiter(Delimiter::Asterisk)
            | Token::Delimiter(Delimiter::EqualEqual) => todo!(),
            Token::EscapedString(_) => todo!(),
            Token::Shell(_) => todo!(),
        }
    }
}

enum LeftDenotation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl LeftDenotation {
    fn left_precedence(&self) -> DenotationPrecedence {
        match self {
            Self::Add => DenotationPrecedence::Add,
            Self::Subtract => DenotationPrecedence::Add,
            Self::Multiply => DenotationPrecedence::Multiply,
            Self::Divide => DenotationPrecedence::Multiply,
        }
    }
}

impl<'a> TryFrom<&PToken<'a>> for LeftDenotation {
    type Error = ();
    fn try_from(token: &PToken<'a>) -> Result<Self, Self::Error> {
        match token {
            Token::Delimiter(Delimiter::OpenSquareBracket) => todo!(),
            Token::Delimiter(Delimiter::OpenParenthesis) => todo!(),
            Token::Delimiter(Delimiter::HyphenMinus) => Ok(LeftDenotation::Subtract),
            Token::Delimiter(Delimiter::Plus) => Ok(LeftDenotation::Add),
            Token::Delimiter(Delimiter::OpenAngleBracket) => todo!(),
            Token::Delimiter(Delimiter::CloseAngleBracket) => todo!(),
            Token::Delimiter(Delimiter::Comma) => todo!(),
            Token::Delimiter(Delimiter::Period) => todo!(),
            Token::Delimiter(Delimiter::EqualEqual) => todo!(),
            Token::Delimiter(Delimiter::ForwardSlash) => Ok(LeftDenotation::Divide),
            Token::Delimiter(Delimiter::Asterisk) => Ok(LeftDenotation::Multiply),
            // not left denotation
            Token::Lexeme(Identifier(_))
            | Token::Lexeme(Lexeme::LexTrue)
            | Token::Lexeme(Lexeme::LexFalse)
            | Token::Number(Number::Binary(_))
            | Token::Number(Number::Hex(_))
            | Token::Number(Number::Unsigned(_))
            | Token::Number(Number::Signed(_))
            | Token::Number(Number::Integer(_))
            | Token::Number(Number::Float(_))
            | Token::RawString(_)
            | Token::Lexeme(Lexeme::LexUnit)
            | Token::Lexeme(Lexeme::LexOk)
            | Token::Lexeme(Lexeme::LexErr)
            | Token::Lexeme(Lexeme::LexSome)
            | Token::Lexeme(Lexeme::LexNone)
            | Token::Lexeme(Lexeme::LexCd)
            | Token::Lexeme(Lexeme::LexProcess)
            | Token::Lexeme(Lexeme::LexRun)
            | Token::Lexeme(Lexeme::LexSpawn)
            | Token::Lexeme(Lexeme::LexWait)
            | Token::Lexeme(Lexeme::LexDrop)
            | Token::Lexeme(Lexeme::LexLet)
            | Token::Lexeme(Lexeme::LexLoop)
            | Token::Lexeme(Lexeme::LexFor)
            | Token::Lexeme(Lexeme::LexWhile)
            | Token::Lexeme(Lexeme::LexIf)
            | Token::Lexeme(Lexeme::LexFn)
            | Token::Lexeme(Lexeme::LexI64)
            | Token::Lexeme(Lexeme::LexU64)
            | Token::Lexeme(Lexeme::LexBool)
            | Token::Lexeme(Lexeme::LexString)
            | Token::Lexeme(Lexeme::LexList)
            | Token::Lexeme(Lexeme::LexMap)
            | Token::Lexeme(Lexeme::LexOption)
            | Token::Lexeme(Lexeme::LexResult)
            | Token::Lexeme(Lexeme::LexJcmd)
            | Token::Lexeme(Lexeme::LexJhandle)
            | Token::Lexeme(Lexeme::LexJexit)
            | Token::Lexeme(Lexeme::LexFhandle)
            | Token::Delimiter(Delimiter::OpenBrace)
            | Token::Delimiter(Delimiter::CloseBrace)
            | Token::Delimiter(Delimiter::CloseSquareBracket)
            | Token::Delimiter(Delimiter::CloseParenthesis)
            | Token::Delimiter(Delimiter::OpenBraceBracket)
            | Token::Delimiter(Delimiter::CloseBracketBrace)
            | Token::Delimiter(Delimiter::Semicolon)
            | Token::Delimiter(Delimiter::Colon)
            | Token::Delimiter(Delimiter::Equal)
            | Token::Delimiter(Delimiter::BackSlash)
            | Token::EscapedString(_)
            | Token::Shell(_) => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DenotationPrecedence {
    None,
    Add,
    Multiply,
}

#[derive(Debug)]
pub enum ParserError<L, F> {
    Incomplete,
    Lexer(L),
    Fatal(F),
}

#[derive(Debug)]
pub enum FatalParserError<N> {
    NUD(N),
}

pub struct Parser<'a, I, E>
where
    I: Iterator<Item = Result<PToken<'a>, E>>,
{
    iter: NPeekable<I, 2>,
}

impl<'a, I, E> Parser<'a, I, E>
where
    I: Iterator<Item = Result<PToken<'a>, E>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: NPeekable::new(iter),
        }
    }
    pub fn parse(&mut self) -> Option<Result<AST<'a>, ParserError<E, FatalParserError<()>>>> {
        match self.iter.next()? {
            Err(e) => Some(Err(ParserError::Lexer(e))),
            Ok(tok) => Some(self.parse_expression(tok, DenotationPrecedence::None)),
        }
    }
    fn parse_nud(
        &mut self,
        nud: NullDenotation<'a>,
    ) -> Result<AST<'a>, ParserError<E, FatalParserError<()>>> {
        match nud {
            NullDenotation::BOOL(boolean) => Ok(AST::Literal(Literal::BOOL(boolean))),
            NullDenotation::I64(number) => Ok(AST::Literal(Literal::I64(number))),
            NullDenotation::U64(number) => Ok(AST::Literal(Literal::U64(number))),
            NullDenotation::F64(number) => Ok(AST::Literal(Literal::F64(number))),
            NullDenotation::STRING(string) => Ok(AST::Literal(Literal::STRING(string))),
            NullDenotation::IDENTIFIER(identifier) => Ok(AST::Identifier(identifier)),
            NullDenotation::Negate => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                match tok {
                    Token::Number(Number::Signed(number_string))
                    | Token::Number(Number::Integer(number_string)) => {
                        match i64::from_str_radix(&format!("-{number_string}"), 10) {
                            Ok(number) => Ok(AST::Literal(Literal::I64(number))),
                            Err(_e) => todo!(),
                        }
                    }
                    tok => {
                        let right = self.parse_expression(tok, DenotationPrecedence::Add)?;
                        Ok(AST::Operation(Operation::Negate(Box::new(right))))
                    }
                }
            }
        }
    }
    fn parse_led(
        &mut self,
        left: AST<'a>,
        led: LeftDenotation,
    ) -> Result<AST<'a>, ParserError<E, FatalParserError<()>>> {
        match led {
            LeftDenotation::Add => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let right = self.parse_expression(tok, DenotationPrecedence::Add)?;
                Ok(AST::Operation(Operation::Add(Box::new((left, right)))))
            }
            LeftDenotation::Subtract => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let right = self.parse_expression(tok, DenotationPrecedence::Add)?;
                Ok(AST::Operation(Operation::Subtract(Box::new((left, right)))))
            }
            LeftDenotation::Multiply => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let right = self.parse_expression(tok, DenotationPrecedence::Multiply)?;
                Ok(AST::Operation(Operation::Multiply(Box::new((left, right)))))
            }
            LeftDenotation::Divide => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let right = self.parse_expression(tok, DenotationPrecedence::Multiply)?;
                Ok(AST::Operation(Operation::Divide(Box::new((left, right)))))
            }
        }
    }
    fn parse_expression(
        &mut self,
        tok: PToken<'a>,
        precedence: DenotationPrecedence,
    ) -> Result<AST<'a>, ParserError<E, FatalParserError<()>>> {
        let mut left = match NullDenotation::try_from(tok) {
            Err(e) => return Err(ParserError::Fatal(FatalParserError::NUD(e))),
            Ok(nud) => self.parse_nud(nud)?,
        };
        loop {
            let led = match self.iter.peek(0) {
                None => unreachable!(),
                Some(None) => break,
                Some(Some(Err(_))) => {
                    return Err(ParserError::Lexer(self.iter.next().unwrap().err().unwrap()));
                }
                Some(Some(Ok(tok))) => match LeftDenotation::try_from(tok) {
                    Err(()) => break,
                    Ok(led) => led,
                },
            };
            if precedence >= led.left_precedence() {
                break;
            }
            let _ = self.iter.next();
            left = self.parse_led(left, led)?;
        }
        Ok(left)
    }
}

impl<'a, I, E> Iterator for Parser<'a, I, E>
where
    I: Iterator<Item = Result<PToken<'a>, E>>,
{
    type Item = Result<AST<'a>, ParserError<E, FatalParserError<()>>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

impl<'a, I, E> FusedIterator for Parser<'a, I, E> where I: Iterator<Item = Result<PToken<'a>, E>> {}
