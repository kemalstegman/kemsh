use std::{
    // collections::HashMap,
    io::{BufRead, BufReader, Write, stdin, stdout},
    // iter::Peekable,
};

use crate::{
    executor::instruction::{Instruction, execute_instruction},
    lexer::{LexError, Lexer, Token},
    parser::{InstructionBridge, ParseError, Parser, TokenBridge},
};

mod executor;
mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout().lock();
    let mut stdin_reader = BufReader::new(stdin().lock());
    let mut s = String::new();
    let mut variable_environment = executor::variables::Environment::new_with_default_globals();
    print!(" ");
    'ic: loop {
        print!("{:?} > ", std::env::current_dir()?);
        stdout.flush().unwrap();
        stdin_reader.read_line(&mut s).unwrap();

        match verb_instructions(s.chars()) {
            Err(err) => {
                eprintln!("{err:?}");
                return Ok(());
            }
            Ok(instructions) => {
                for instruction in instructions {
                    match execute_instruction(instruction, &mut variable_environment) {
                        Ok(()) => (),
                        Err(err) => {
                            eprintln!("{err:?}");
                            return Ok(());
                        }
                    }
                }
                s.clear();
            }
        }
        // let char_vec: Vec<char> = s.chars().collect();
        // match lexer::lex(char_vec.iter().copied().peekable()) {
        //     Ok(Some(v)) => {
        //         // println!("{v:?}");
        //         match parser::parse(v.into_iter().peekable()) {
        //             Err(e) => {
        //                 println!("{e:?}");
        //                 s.clear();
        //                 print!(" ");
        //                 continue 'ic;
        //             }
        //             Ok(None) => {
        //                 print!(">");
        //                 continue 'ic;
        //             }
        //             Ok(Some(instructions)) => {
        //                 s.clear();
        //                 for instruction in instructions {
        //                     match executor::instruction::execute_instruction(
        //                         instruction,
        //                         &mut variable_environment,
        //                     ) {
        //                         Ok(()) => print!(" "),
        //                         Err(e) => {
        //                             println!("{e:?}");
        //                             s.clear();
        //                             print!(" ");
        //                             continue 'ic;
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         println!("{e:?}");
        //         s.clear();
        //         print!(" ");
        //         continue 'ic;
        //     }
        //     Ok(None) => {
        //         print!(">");
        //         continue 'ic;
        //     }
        // }
    }
    // Ok(())
}

#[derive(Debug)]
struct VerbInstructionsError {
    message: String,
}

fn verb_instructions<I>(iter: I) -> Result<Vec<Instruction>, VerbInstructionsError>
where
    I: Iterator<Item = char>,
{
    let lexer = Lexer::new(iter);
    let mut token_bridge = TokenBridge::new(lexer);
    let parser = Parser::new(&mut token_bridge);
    let mut instruction_bridge = InstructionBridge::new(parser);
    let instructions = (&mut instruction_bridge).collect::<Vec<Instruction>>();
    let parser_error = instruction_bridge.take_error();
    let lexer_error = token_bridge.take_error();
    if let Some(err) = lexer_error {
        return Err(VerbInstructionsError {
            message: format!("{err:?}"),
        });
    }
    if let Some(err) = parser_error {
        return Err(VerbInstructionsError {
            message: format!("{err:?}"),
        });
    }
    Ok(instructions)
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
