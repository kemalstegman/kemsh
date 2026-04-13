use std::{
    convert::Infallible,
    io::{BufRead, BufReader, Write, stdin, stdout},
};

use kemsh::{
    ast::Expression,
    executor::{Executor, ExecutorError},
    syntax::{lexer::LexerError, parser::ParserError, syntax},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::exit(repl()? as i32)
}

fn repl() -> Result<i64, Box<dyn std::error::Error>> {
    let mut stdout = stdout().lock();
    let mut stdin_reader = BufReader::new(stdin().lock());
    let mut s = String::new();
    let mut executor = Executor::new();
    print!("KEMSH > ");
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
                print!("KEMSH > ");
                continue;
            }
            Ok(expressions) => expressions,
        };
        for expression in expressions {
            match executor.execute_top_level_expression(expression) {
                Ok(()) => (),
                Err(ExecutorError::Exit(status)) => return Ok(status),
                Err(err) => {
                    println!("execution error: {err:?}");
                    break;
                }
            }
        }
        print!("KEMSH > ");
        // print!("{:?} > ", std::env::current_dir()?);
        s.clear()
    }
}
