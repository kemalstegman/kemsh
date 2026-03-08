use std::{
    // collections::HashMap,
    io::{BufRead, BufReader, Write, stdin, stdout},
    // iter::Peekable,
};

mod executor;
mod lexer;
mod parser;

mod unit_instructions;

fn main() {
    let mut stdout = stdout().lock();
    print!("Hello, world! > ");
    stdout.flush().unwrap();
    let mut stdin_reader = BufReader::new(stdin().lock());
    let mut s = String::new();
    let mut variable_environment = executor::VariableEnvironment::new();
    variable_environment.push_scope();
    loop {
        stdin_reader.read_line(&mut s).unwrap();
        let char_vec: Vec<char> = s.chars().collect();
        match lexer::lex(char_vec.iter().copied().peekable()) {
            Ok(Some(v)) => {
                println!("{v:?}");
                match parser::parse(v.into_iter().peekable()) {
                    Err(e) => {
                        println!("{e:?}");
                        break;
                    }
                    Ok(None) => continue,
                    Ok(Some(instructions)) => {
                        s.clear();
                        for instruction in instructions {
                            match executor::execute_instruction(
                                instruction,
                                &mut variable_environment,
                            ) {
                                Ok(()) => (),
                                Err(e) => {
                                    println!("{e:?}");
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("{e:?}");
                break;
            }
            Ok(None) => continue,
        }
    }
}

// enum VariableValue {
//     I32(i32),
// }

// #[derive(Debug)]
// enum LexerToken {
//     Identifier(String),
//     Symbol(char),
//     String(String),
// }

// #[derive(Debug)]
// struct LexerError {
//     message: String,
// }

// enum LexerResult {
//     Ok(Vec<LexerToken>),
//     Incomplete,
//     Error(LexerError),
// }

// fn lex_input(mut input_stream: Peekable<impl Iterator<Item = char>>) -> LexerResult {
//     let mut tokens: Vec<LexerToken> = Vec::new();

//     while let Some(c) = input_stream.next() {
//         match c {
//             ' ' | '\t' | '\n' | '\r' => (),
//             c @ ('a'..='z' | 'A'..='Z' | '_') => {
//                 let mut identifier = String::from(c);
//                 loop {
//                     let Some(&c) = input_stream.peek() else {
//                         return LexerResult::Incomplete;
//                     };
//                     match c {
//                         c @ ('a'..='z' | 'A'..='Z' | '_' | '0'..='9') => {
//                             identifier.push(c);
//                             input_stream.next();
//                         }
//                         _ => break,
//                     }
//                 }
//                 tokens.push(LexerToken::Identifier(identifier));
//             }
//             c @ ('=' | '+' | '-' | '*' | '/' | '{' | '}') => {
//                 tokens.push(LexerToken::Symbol(c));
//             }
//             c => {
//                 return LexerResult::Error(LexerError {
//                     message: format!("Unknown character: {c:?}"),
//                 });
//             }
//         }
//     }

//     LexerResult::Ok(tokens)
// }
