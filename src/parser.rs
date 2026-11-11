use std::iter::FusedIterator;

use crate::{
    ast::{AST, IOperation, POperation, SourceBorrowedText, SpecialOperation, Value, ValueKind},
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
    UNIT,
    IDENTIFIER(&'a str),
    Negate,
    Ok,
    Err,
    Some,
    NONE,
    ChangeDirectory,
    Process,
    Run,
    Spawn,
    Wait,
    Drop,
    Let,
    Loop,
    For,
    While,
    If,
    Fn,
    ListOpen,
    MapOpen,
    ScopeOpen,
    ParenthesisOpen,
    Shell(&'a str),
}

#[derive(Debug)]
pub enum NullDenotationError<'a> {
    Unexpected(PToken<'a>),
    IntError(std::num::ParseIntError),
    FloatError(std::num::ParseFloatError),
}

impl<'a> TryFrom<PToken<'a>> for NullDenotation<'a> {
    type Error = NullDenotationError<'a>;
    fn try_from(token: PToken<'a>) -> Result<Self, Self::Error> {
        match token {
            // primitive values
            Token::Lexeme(Identifier(identifier)) => Ok(NullDenotation::IDENTIFIER(identifier)),
            Token::Lexeme(Lexeme::LexTrue) => Ok(NullDenotation::BOOL(true)),
            Token::Lexeme(Lexeme::LexFalse) => Ok(NullDenotation::BOOL(false)),
            Token::Number(Number::Binary(number_string)) => {
                match u64::from_str_radix(number_string, 2) {
                    Ok(number) => Ok(NullDenotation::U64(number)),
                    Err(e) => Err(NullDenotationError::IntError(e)),
                }
            }
            Token::Number(Number::Hex(number_string)) => {
                match u64::from_str_radix(number_string, 16) {
                    Ok(number) => Ok(NullDenotation::U64(number)),
                    Err(e) => Err(NullDenotationError::IntError(e)),
                }
            }
            Token::Number(Number::Unsigned(number_string)) => {
                match u64::from_str_radix(number_string, 10) {
                    Ok(number) => Ok(NullDenotation::U64(number)),
                    Err(e) => Err(NullDenotationError::IntError(e)),
                }
            }
            Token::Number(Number::Signed(number_string))
            | Token::Number(Number::Integer(number_string)) => {
                match i64::from_str_radix(number_string, 10) {
                    Ok(number) => Ok(NullDenotation::I64(number)),
                    Err(e) => Err(NullDenotationError::IntError(e)),
                }
            }
            Token::Number(Number::Float(number_string)) => match number_string.parse::<f64>() {
                Ok(number) => Ok(NullDenotation::F64(number)),
                Err(e) => Err(NullDenotationError::FloatError(e)),
            },
            Token::RawString(string) => Ok(NullDenotation::STRING(string)),
            Token::EscapedString(_) => todo!(),
            Token::Shell(shell) => Ok(NullDenotation::Shell(shell)),
            Token::Lexeme(Lexeme::LexUnit) => Ok(NullDenotation::UNIT),
            Token::Lexeme(Lexeme::LexNone) => Ok(NullDenotation::NONE),
            // operators
            Token::Lexeme(Lexeme::LexOk) => Ok(NullDenotation::Ok),
            Token::Lexeme(Lexeme::LexErr) => Ok(NullDenotation::Err),
            Token::Lexeme(Lexeme::LexSome) => Ok(NullDenotation::Some),
            Token::Lexeme(Lexeme::LexCd) => Ok(NullDenotation::ChangeDirectory),
            Token::Lexeme(Lexeme::LexProcess) => Ok(NullDenotation::Process),
            Token::Lexeme(Lexeme::LexRun) => Ok(NullDenotation::Run),
            Token::Lexeme(Lexeme::LexSpawn) => Ok(NullDenotation::Spawn),
            Token::Lexeme(Lexeme::LexWait) => Ok(NullDenotation::Wait),
            Token::Lexeme(Lexeme::LexDrop) => Ok(NullDenotation::Drop),
            // let
            Token::Lexeme(Lexeme::LexLet) => Ok(NullDenotation::Let),
            // control flow
            Token::Lexeme(Lexeme::LexLoop) => Ok(NullDenotation::Loop),
            Token::Lexeme(Lexeme::LexFor) => Ok(NullDenotation::For),
            Token::Lexeme(Lexeme::LexWhile) => Ok(NullDenotation::While),
            Token::Lexeme(Lexeme::LexIf) => Ok(NullDenotation::If),
            Token::Lexeme(Lexeme::LexFn) => Ok(NullDenotation::Fn),
            // delimiters
            Token::Delimiter(Delimiter::HyphenMinus) => Ok(NullDenotation::Negate),
            Token::Delimiter(Delimiter::OpenBrace) => Ok(NullDenotation::ScopeOpen),
            Token::Delimiter(Delimiter::OpenSquareBracket) => Ok(NullDenotation::ListOpen),
            Token::Delimiter(Delimiter::OpenParenthesis) => Ok(NullDenotation::ParenthesisOpen),
            Token::Delimiter(Delimiter::OpenBraceBracket) => Ok(NullDenotation::MapOpen),
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
            | Token::Lexeme(Lexeme::LexFhandle) => Err(NullDenotationError::Unexpected(token)),
            // unexpected delimiters
            Token::Delimiter(Delimiter::Plus)
            | Token::Delimiter(Delimiter::OpenAngleBracket)
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
            | Token::Delimiter(Delimiter::EqualEqual) => {
                Err(NullDenotationError::Unexpected(token))
            }
        }
    }
}

enum LeftDenotation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Call,
    Index,
    LessThan,
    GreaterThan,
    Equal,
    Assign,
    Field,
}

impl LeftDenotation {
    fn left_precedence(&self) -> DenotationPrecedence {
        match self {
            Self::Add => DenotationPrecedence::Add,
            Self::Subtract => DenotationPrecedence::Add,
            Self::Multiply => DenotationPrecedence::Multiply,
            Self::Divide => DenotationPrecedence::Multiply,
            Self::Call => DenotationPrecedence::Call,
            Self::Index => DenotationPrecedence::Call,
            Self::LessThan => DenotationPrecedence::Compare,
            Self::GreaterThan => DenotationPrecedence::Compare,
            Self::Equal => DenotationPrecedence::Compare,
            Self::Assign => DenotationPrecedence::Assign,
            Self::Field => DenotationPrecedence::Call,
        }
    }
}

impl<'a> TryFrom<&PToken<'a>> for LeftDenotation {
    type Error = ();
    fn try_from(token: &PToken<'a>) -> Result<Self, Self::Error> {
        match token {
            Token::Delimiter(Delimiter::OpenSquareBracket) => Ok(LeftDenotation::Index),
            Token::Delimiter(Delimiter::OpenParenthesis) => Ok(LeftDenotation::Call),
            Token::Delimiter(Delimiter::HyphenMinus) => Ok(LeftDenotation::Subtract),
            Token::Delimiter(Delimiter::Plus) => Ok(LeftDenotation::Add),
            Token::Delimiter(Delimiter::OpenAngleBracket) => Ok(LeftDenotation::LessThan),
            Token::Delimiter(Delimiter::CloseAngleBracket) => Ok(LeftDenotation::GreaterThan),
            Token::Delimiter(Delimiter::Period) => Ok(LeftDenotation::Field),
            Token::Delimiter(Delimiter::EqualEqual) => Ok(LeftDenotation::Equal),
            Token::Delimiter(Delimiter::Equal) => Ok(LeftDenotation::Assign),
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
            | Token::Delimiter(Delimiter::Comma)
            | Token::Delimiter(Delimiter::CloseBrace)
            | Token::Delimiter(Delimiter::CloseSquareBracket)
            | Token::Delimiter(Delimiter::CloseParenthesis)
            | Token::Delimiter(Delimiter::OpenBraceBracket)
            | Token::Delimiter(Delimiter::CloseBracketBrace)
            | Token::Delimiter(Delimiter::Semicolon)
            | Token::Delimiter(Delimiter::Colon)
            | Token::Delimiter(Delimiter::BackSlash)
            | Token::EscapedString(_)
            | Token::Shell(_) => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum DenotationPrecedence {
    None,
    AssignRight,
    Assign,
    Compare,
    Add,
    Multiply,
    Prefix,
    Call,
}

#[derive(Debug)]
pub enum ParserError<L, F> {
    Incomplete,
    Lexer(L),
    Fatal(F),
}

#[derive(Debug)]
pub enum FatalParserError<'a, N> {
    NUD(N),
    ExpectedSemicolon(PToken<'a>),
    ExpectedClosingParenthesis(PToken<'a>),
    ExpectedClosingSquareBracket(PToken<'a>),
    ExpectedType(PToken<'a>),
    ExpectedIdentifier(PToken<'a>),
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
    pub fn parse(
        &mut self,
    ) -> Option<
        Result<
            AST<'a, SourceBorrowedText>,
            ParserError<E, FatalParserError<'a, NullDenotationError<'a>>>,
        >,
    > {
        match self.iter.next()? {
            Err(e) => Some(Err(ParserError::Lexer(e))),
            Ok(tok) => match self.parse_expression(tok, DenotationPrecedence::None, 0) {
                Err(e) => Some(Err(e)),
                Ok(ast) => match self.iter.next() {
                    Some(Err(e)) => Some(Err(ParserError::Lexer(e))),
                    None | Some(Ok(Token::Delimiter(Delimiter::Semicolon))) => Some(Ok(ast)),
                    Some(Ok(token)) => Some(Err(ParserError::Fatal(
                        FatalParserError::ExpectedSemicolon(token),
                    ))),
                },
            },
        }
    }
    fn parse_nud(
        &mut self,
        nud: NullDenotation<'a>,
        depth: usize,
    ) -> Result<
        AST<'a, SourceBorrowedText>,
        ParserError<E, FatalParserError<'a, NullDenotationError<'a>>>,
    > {
        match nud {
            NullDenotation::BOOL(boolean) => Ok(AST::Value(Value::BOOL(boolean))),
            NullDenotation::I64(number) => Ok(AST::Value(Value::I64(number))),
            NullDenotation::U64(number) => Ok(AST::Value(Value::U64(number))),
            NullDenotation::F64(number) => Ok(AST::Value(Value::F64(number))),
            NullDenotation::STRING(string) => Ok(AST::Value(Value::STRING(string))),
            NullDenotation::IDENTIFIER(identifier) => Ok(AST::Identifier(identifier)),
            NullDenotation::UNIT => Ok(AST::Value(Value::UNIT)),
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
                            Ok(number) => Ok(AST::Value(Value::I64(number))),
                            Err(e) => Err(ParserError::Fatal(FatalParserError::NUD(
                                NullDenotationError::IntError(e),
                            ))),
                        }
                    }
                    tok => {
                        let rhs = self.parse_expression(tok, DenotationPrecedence::Add, depth)?;
                        Ok(AST::POperation(Box::new(POperation::Negate { rhs })))
                    }
                }
            }
            NullDenotation::Ok => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Ok {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Err => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Err {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Some => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Some {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::NONE => Ok(AST::Value(Value::Option(None))),
            NullDenotation::ChangeDirectory => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::ChangeDirectory {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Process => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Process {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Run => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Run {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Spawn => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Spawn {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Wait => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                Ok(AST::POperation(Box::new(POperation::Wait {
                    rhs: self.parse_expression(tok, DenotationPrecedence::Prefix, depth)?,
                })))
            }
            NullDenotation::Drop => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                match tok {
                    Token::Lexeme(Lexeme::Identifier(identifier)) => {
                        Ok(AST::SpecialOperation(SpecialOperation::Drop {
                            rhs_iden: identifier,
                        }))
                    }
                    tok => {
                        return Err(ParserError::Fatal(FatalParserError::ExpectedIdentifier(
                            tok,
                        )));
                    }
                }
            }
            NullDenotation::Let => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let identifier = match tok {
                    Token::Lexeme(Lexeme::Identifier(identifier)) => identifier,
                    tok => {
                        return Err(ParserError::Fatal(FatalParserError::ExpectedIdentifier(
                            tok,
                        )));
                    }
                };
                let r#type = match self.iter.peek(0).unwrap().ok_or(ParserError::Incomplete)? {
                    Err(_) => {
                        return Err(ParserError::Lexer(self.iter.next().unwrap().err().unwrap()));
                    }
                    Ok(Token::Delimiter(Delimiter::Colon)) => {
                        let _ = self.iter.next();
                        Some(self.parse_type()?)
                    }
                    Ok(_) => None,
                };
                let expression = match self.iter.peek(0).unwrap().ok_or(ParserError::Incomplete)? {
                    Err(_) => {
                        return Err(ParserError::Lexer(self.iter.next().unwrap().err().unwrap()));
                    }
                    Ok(Token::Delimiter(Delimiter::Equal)) => {
                        let _ = self.iter.next();
                        let tok = self
                            .iter
                            .next()
                            .ok_or(ParserError::Incomplete)?
                            .map_err(|e| ParserError::Lexer(e))?;
                        Some(Box::new(self.parse_expression(
                            tok,
                            DenotationPrecedence::AssignRight,
                            depth,
                        )?))
                    }
                    Ok(_) => None,
                };
                Ok(AST::SpecialOperation(SpecialOperation::Let {
                    identifier,
                    r#type,
                    expression,
                }))
            }
            NullDenotation::Loop => todo!(),
            NullDenotation::For => todo!(),
            NullDenotation::While => todo!(),
            NullDenotation::If => todo!(),
            NullDenotation::Fn => todo!(),
            NullDenotation::ListOpen => todo!(),
            NullDenotation::MapOpen => todo!(),
            NullDenotation::ScopeOpen => todo!(),
            NullDenotation::ParenthesisOpen => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let expression = self.parse_expression(tok, DenotationPrecedence::None, depth)?;
                match self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?
                {
                    Token::Delimiter(Delimiter::CloseParenthesis) => Ok(expression),
                    tok => Err(ParserError::Fatal(
                        FatalParserError::ExpectedClosingParenthesis(tok),
                    )),
                }
            }
            NullDenotation::Shell(shell) => {
                Ok(AST::SpecialOperation(SpecialOperation::Shell(shell)))
            }
        }
    }
    fn parse_type(
        &mut self,
    ) -> Result<ValueKind, ParserError<E, FatalParserError<'a, NullDenotationError<'a>>>> {
        let tok = self
            .iter
            .next()
            .ok_or(ParserError::Incomplete)?
            .map_err(|e| ParserError::Lexer(e))?;
        match tok {
            Token::Lexeme(Lexeme::LexI64) => Ok(ValueKind::I64),
            Token::Lexeme(Lexeme::LexU64) => Ok(ValueKind::U64),
            Token::Lexeme(Lexeme::LexBool) => Ok(ValueKind::BOOL),
            Token::Lexeme(Lexeme::LexString) => Ok(ValueKind::STRING),
            Token::Lexeme(Lexeme::LexList) => Ok(ValueKind::List),
            Token::Lexeme(Lexeme::LexMap) => Ok(ValueKind::Map),
            Token::Lexeme(Lexeme::LexOption) => Ok(ValueKind::Option),
            Token::Lexeme(Lexeme::LexResult) => Ok(ValueKind::Result),
            Token::Lexeme(Lexeme::LexJcmd) => Ok(ValueKind::Jcmd),
            Token::Lexeme(Lexeme::LexJhandle) => Ok(ValueKind::Jhandle),
            Token::Lexeme(Lexeme::LexJexit) => Ok(ValueKind::Jexit),
            Token::Lexeme(Lexeme::LexFhandle) => Ok(ValueKind::Fhandle),
            tok => Err(ParserError::Fatal(FatalParserError::ExpectedType(tok))),
        }
    }
    fn parse_led(
        &mut self,
        lhs: AST<'a, SourceBorrowedText>,
        led: LeftDenotation,
        depth: usize,
    ) -> Result<
        AST<'a, SourceBorrowedText>,
        ParserError<E, FatalParserError<'a, NullDenotationError<'a>>>,
    > {
        match led {
            LeftDenotation::Add => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Add, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::Add { lhs, rhs })))
            }
            LeftDenotation::Subtract => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Add, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::Subtract { lhs, rhs })))
            }
            LeftDenotation::Multiply => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Multiply, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::Multiply { lhs, rhs })))
            }
            LeftDenotation::Divide => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Multiply, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::Divide { lhs, rhs })))
            }
            LeftDenotation::Call => todo!(),
            LeftDenotation::Index => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::None, depth)?;
                match self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?
                {
                    Token::Delimiter(Delimiter::CloseSquareBracket) => {
                        Ok(AST::IOperation(Box::new(IOperation::Index { lhs, rhs })))
                    }
                    tok => Err(ParserError::Fatal(
                        FatalParserError::ExpectedClosingSquareBracket(tok),
                    )),
                }
            }
            LeftDenotation::LessThan => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Compare, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::LessThan { lhs, rhs })))
            }
            LeftDenotation::GreaterThan => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Compare, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::GreaterThan {
                    lhs,
                    rhs,
                })))
            }
            LeftDenotation::Equal => {
                let tok = self
                    .iter
                    .next()
                    .ok_or(ParserError::Incomplete)?
                    .map_err(|e| ParserError::Lexer(e))?;
                let rhs = self.parse_expression(tok, DenotationPrecedence::Compare, depth)?;
                Ok(AST::IOperation(Box::new(IOperation::Equal { lhs, rhs })))
            }
            LeftDenotation::Assign => todo!(),
            LeftDenotation::Field => todo!(),
        }
    }
    fn parse_expression(
        &mut self,
        tok: PToken<'a>,
        precedence: DenotationPrecedence,
        depth: usize,
    ) -> Result<
        AST<'a, SourceBorrowedText>,
        ParserError<E, FatalParserError<'a, NullDenotationError<'a>>>,
    > {
        if depth > 1000 {
            panic!("maximum recursion depth reached")
        }
        let mut left = match NullDenotation::try_from(tok) {
            Err(e) => return Err(ParserError::Fatal(FatalParserError::NUD(e))),
            Ok(nud) => self.parse_nud(nud, depth + 1)?,
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
            left = self.parse_led(left, led, depth + 1)?;
        }
        Ok(left)
    }
}

impl<'a, I, E> Iterator for Parser<'a, I, E>
where
    I: Iterator<Item = Result<PToken<'a>, E>>,
{
    type Item = Result<
        AST<'a, SourceBorrowedText>,
        ParserError<E, FatalParserError<'a, NullDenotationError<'a>>>,
    >;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse()
    }
}

impl<'a, I, E> FusedIterator for Parser<'a, I, E> where I: Iterator<Item = Result<PToken<'a>, E>> {}
