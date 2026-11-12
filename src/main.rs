use std::io::BufRead;

use kemsh::{
    evaluation::{Environment, evaluate},
    lexer::{
        Lexer, LexerError, StandardDelimiterLexer, StandardEscapedStringLexer, StandardLexemeLexer,
        StandardNumberLexer, StandardRawStringLexer, StandardShellLexer,
    },
    parser::{Parser, ParserError},
};

fn main() {
    let mut environment = Environment::new();
    let mut inp = String::new();
    'ml: loop {
        inp.clear();
        'rl: loop {
            std::io::stdin().lock().read_line(&mut inp).unwrap();
            if inp.len() == 0 {
                break 'ml;
            }
            for res in Parser::new(Lexer::<
                StandardLexemeLexer,
                StandardNumberLexer,
                StandardDelimiterLexer,
                StandardRawStringLexer,
                StandardEscapedStringLexer,
                StandardShellLexer,
            >::new(inp.as_str()))
            {
                match res {
                    Err(ParserError::Incomplete | ParserError::Lexer(LexerError::Incomplete)) => {
                        continue 'rl;
                    }
                    Err(_) => {
                        println!("{res:?}");
                        continue 'ml;
                    }
                    Ok(_) => (),
                }
            }
            break;
        }
        for exp_res in Parser::new(Lexer::<
            StandardLexemeLexer,
            StandardNumberLexer,
            StandardDelimiterLexer,
            StandardRawStringLexer,
            StandardEscapedStringLexer,
            StandardShellLexer,
        >::new(inp.as_str()))
        .into_iter()
        {
            if let Ok(exp) = exp_res {
                println!("{:?}", evaluate(&exp, &mut environment));
            } else {
                println!("{exp_res:#?}");
                break;
            }
        }
    }
}
