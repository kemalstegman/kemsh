pub mod lexer;
use lexer::{LexError, Lexer};

pub mod parser;
use parser::{ParseError, Parser};

use crate::instructions::Instruction;

pub enum AnalyzerError<E> {
    CharInput(E),
    Lexer(lexer::LexError),
    Parser(parser::ParseError),
    Incomplete,
}

pub fn analyze<I, E>(iter: I) -> impl Iterator<Item = Result<Instruction, AnalyzerError<E>>>
where
    I: IntoIterator<Item = Result<char, E>>,
{
    Parser::new(Lexer::new(
        iter.into_iter()
            .map(|res| res.map_err(|err| AnalyzerError::CharInput(err))),
    ))
}
