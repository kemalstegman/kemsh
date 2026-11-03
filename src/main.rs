use std::io::Read;

use kemsh::{
    lexer::{
        Lexer, StandardDelimiterLexer, StandardEscapedStringLexer, StandardLexemeLexer,
        StandardNumberLexer, StandardRawStringLexer, StandardShellLexer,
    },
    parser::Parser,
};

fn main() {
    let mut inp = String::new();
    std::io::stdin().lock().read_to_string(&mut inp).unwrap();
    let lexer = Lexer::<
        StandardLexemeLexer,
        StandardNumberLexer,
        StandardDelimiterLexer,
        StandardRawStringLexer,
        StandardEscapedStringLexer,
        StandardShellLexer,
    >::new(inp.as_str());
    for exp_res in Parser::new(lexer).into_iter() {
        println!("{exp_res:?}");
    }
}
