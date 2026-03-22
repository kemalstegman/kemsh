use std::{
    // collections::HashMap,
    io::{BufRead, BufReader, Write, stdin, stdout},
    // iter::Peekable,
};

use kemsh::{
    abstract_syntax_tree::Instruction,
    analyzer::{AnalyzerError, analyze},
    executor::Executor,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let instructions = match analyze(s.chars().map(|ch| Ok::<char, ()>(ch)))
            .collect::<Result<Vec<Instruction>, AnalyzerError<()>>>()
        {
            Err(AnalyzerError::Incomplete) => {
                print!(">> ");
                continue;
            }
            Err(err) => return Err(format!("analyzer error: {err:?}").into()),
            Ok(instructions) => instructions,
        };
        for instruction in instructions {
            match executor.execute_instruction(instruction) {
                Ok(()) => (),
                Err(err) => return Err(format!("execution error: {err:?}").into()),
            }
        }
        print!("{:?} > ", std::env::current_dir()?);
        s.clear()
    }
}
