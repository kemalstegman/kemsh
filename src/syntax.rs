pub mod lexer;
pub mod parser;

use itertools::peek_nth;

use crate::{
    ast::Expression,
    syntax::{
        lexer::{Lexer, LexerError},
        parser::{Parser, ParserError},
    },
};

pub fn syntax<I, E>(iter: I) -> impl Iterator<Item = Result<Expression, ParserError<LexerError<E>>>>
where
    I: Iterator<Item = Result<char, E>>,
{
    Parser::new(peek_nth(
        Lexer::new(peek_nth(iter.map(|x| x.map_err(|e| LexerError::Source(e)))))
            .map(|x| x.map_err(|e| ParserError::Source(e))),
    ))
}

// let var;
// let var: [type];
// let var = [expr];
// let var: [type] = [expr];
// var = [expr];
// echo [expr];
// cd [expr: string];
// if [expr: boolean] {}
// loop {}
// while [expr: boolean] {}
// block [expr: pcmd]; -> pexit
// spawn [expr: pcmd]; -> phandle
// process [expr: string];
// let var = fn() -> [type] {}
// let var = fn() {}
// fn var(){} -> [type] {}
// fn var(){} {}
// ??for??
// ??return [?expr];??
// ??break [?expr];??
