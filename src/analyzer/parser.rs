use std::iter::Peekable;

pub use super::AnalyzerError;

use super::super::instructions::Instruction;
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
