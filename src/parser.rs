use std::iter::FusedIterator;

use itertools::{PeekNth, peek_nth};

use crate::lexer::{
    Delimiter,
    Lexeme::{self, Identifier},
    Number, Token,
};

type PToken<'a> = Token<Lexeme<'a>, Number<'a>, Delimiter, &'a str, (), &'a str>;

enum NullDenotation<'a> {
    Number(&'a str),
}

impl<'a> TryFrom<PToken<'a>> for NullDenotation<'a> {
    type Error = ();
    fn try_from(token: PToken<'a>) -> Result<Self, Self::Error> {
        match token {
            Token::Lexeme(Identifier(_iden)) => todo!(),
            Token::Lexeme(Lexeme::LexTrue) => todo!(),
            Token::Lexeme(Lexeme::LexFalse) => todo!(),
            Token::Number(Number::Binary(_num)) => todo!(),
            Token::Number(Number::Hex(_num)) => todo!(),
            Token::Number(Number::Unsigned(_num)) => todo!(),
            Token::Number(Number::Signed(num)) | Token::Number(Number::Integer(num)) => {
                Ok(NullDenotation::Number(num))
            }
            Token::Number(Number::Float(_num)) => todo!(),
            Token::RawString(_string) => todo!(),
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
            Token::Delimiter(Delimiter::HyphenMinus) => todo!(),
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
    Multiply,
}

impl LeftDenotation {
    fn left_precedence(&self) -> DenotationPrecedence {
        match self {
            Self::Add => DenotationPrecedence::Add,
            Self::Multiply => DenotationPrecedence::Multiply,
        }
    }
}

impl<'a> TryFrom<&PToken<'a>> for LeftDenotation {
    type Error = ();
    fn try_from(token: &PToken<'a>) -> Result<Self, Self::Error> {
        match token {
            Token::Delimiter(Delimiter::OpenSquareBracket) => todo!(),
            Token::Delimiter(Delimiter::OpenParenthesis) => todo!(),
            Token::Delimiter(Delimiter::HyphenMinus) => todo!(),
            Token::Delimiter(Delimiter::Plus) => Ok(Self::Add),
            Token::Delimiter(Delimiter::OpenAngleBracket) => todo!(),
            Token::Delimiter(Delimiter::CloseAngleBracket) => todo!(),
            Token::Delimiter(Delimiter::Comma) => todo!(),
            Token::Delimiter(Delimiter::Period) => todo!(),
            Token::Delimiter(Delimiter::EqualEqual) => todo!(),
            Token::Delimiter(Delimiter::ForwardSlash) => todo!(),
            Token::Delimiter(Delimiter::Asterisk) => Ok(Self::Multiply),
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
pub enum TooBasicExpression<'a> {
    Number(&'a str),
    Add(Box<(TooBasicExpression<'a>, TooBasicExpression<'a>)>),
    Multiply(Box<(TooBasicExpression<'a>, TooBasicExpression<'a>)>),
}

pub struct Parser<'a, I, E>
where
    I: Iterator<Item = Result<PToken<'a>, E>>,
{
    iter: PeekNth<I>,
}

impl<'a, I, E> Parser<'a, I, E>
where
    I: Iterator<Item = Result<PToken<'a>, E>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: peek_nth(iter),
        }
    }
    pub fn parse(&mut self) -> Option<Result<TooBasicExpression<'a>, E>> {
        match self.iter.next()? {
            Err(e) => Some(Err(e)),
            Ok(tok) => Some(self.parse_expression(tok, DenotationPrecedence::None)),
        }
    }
    fn parse_nud(&mut self, nud: NullDenotation<'a>) -> Result<TooBasicExpression<'a>, E> {
        match nud {
            NullDenotation::Number(num) => Ok(TooBasicExpression::Number(num)),
        }
    }
    fn parse_led(
        &mut self,
        left: TooBasicExpression<'a>,
        led: LeftDenotation,
    ) -> Result<TooBasicExpression<'a>, E> {
        match led {
            LeftDenotation::Add => {
                let tok = self.iter.next().unwrap().ok().unwrap();
                Ok(TooBasicExpression::Add(Box::new((
                    left,
                    self.parse_expression(tok, DenotationPrecedence::Add)?,
                ))))
            }
            LeftDenotation::Multiply => {
                let tok = self.iter.next().unwrap().ok().unwrap();
                Ok(TooBasicExpression::Multiply(Box::new((
                    left,
                    self.parse_expression(tok, DenotationPrecedence::Multiply)?,
                ))))
            }
        }
    }
    fn parse_expression(
        &mut self,
        tok: PToken<'a>,
        precedence: DenotationPrecedence,
    ) -> Result<TooBasicExpression<'a>, E> {
        let mut left = match NullDenotation::try_from(tok) {
            Err(()) => todo!(),
            Ok(nud) => self.parse_nud(nud)?,
        };
        loop {
            let led = match self.iter.peek() {
                None => break,
                Some(Err(_)) => return Err(self.iter.next().unwrap().err().unwrap()),
                Some(Ok(tok)) => match LeftDenotation::try_from(tok) {
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
    type Item = Result<TooBasicExpression<'a>, E>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

impl<'a, I, E> FusedIterator for Parser<'a, I, E> where I: Iterator<Item = Result<PToken<'a>, E>> {}
