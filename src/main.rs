use std::{
    convert::Infallible,
    io::{BufRead, BufReader, Write, stdin, stdout},
};

pub mod syntax;
use syntax::{lexer::LexerError, parser::ParserError, syntax};

pub mod ast;
use ast::Expression;

pub mod executor;
use executor::{Executor, ExecutorError};

use crate::executor::concrete::VoidConcrete;

// lookahead iterator
// todo: make own crate
pub mod abstract_lookahead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(repl()? as i32)
}

fn repl() -> Result<i64, Box<dyn std::error::Error>> {
    let mut stdout = stdout().lock();
    let mut stdin_reader = BufReader::new(stdin().lock());
    let mut s = String::new();
    let mut executor = Executor::new();
    print!("KEMSH {} > ", executor.working_directory().display());
    loop {
        stdout.flush()?;
        stdin_reader.read_line(&mut s)?;
        // let mut ins = analyze(s.chars().map(|ch| Ok::<char, ()>(ch)));
        // println!("{:?}", ins.next());
        // println!("{:?}", ins.next());
        // println!("{:?}", ins.next());
        // drop(ins);
        let expressions = match syntax(s.chars().map(|ch| Ok::<char, Infallible>(ch)))
            .collect::<Result<Vec<Expression>, ParserError<LexerError<Infallible>>>>()
        {
            Err(ParserError::Incomplete | ParserError::Source(LexerError::Incomplete)) => {
                print!(">> ");
                continue;
            }
            Err(err) => {
                println!("syntax error: {err:?}");
                s.clear();
                print!("KEMSH {} > ", executor.working_directory().display());
                continue;
            }
            Ok(expressions) => expressions,
        };
        for expression in expressions {
            match executor.execute_expression(&expression) {
                Ok(VoidConcrete::Void) => (),
                Ok(VoidConcrete::Rife(c)) => println!("{:?}", c),
                Err(ExecutorError::Exit(status)) => return Ok(status),
                Err(err) => {
                    println!("execution error: {err:?}");
                    break;
                }
            }
        }
        print!("KEMSH {} > ", executor.working_directory().display());
        // print!("{:?} > ", std::env::current_dir()?);
        s.clear()
    }
}
