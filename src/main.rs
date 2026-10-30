use std::io::Read;

use kemsh::lexer::{
    Lexer, StandardDelimiterLexer, StandardEscapedStringLexer, StandardLexemeLexer,
    StandardNumberLexer, StandardRawStringLexer, StandardShellLexer,
};

fn main() {
    let mut inp = String::new();
    std::io::stdin().lock().read_to_string(&mut inp).unwrap();
    for tok_res in Lexer::<
        StandardLexemeLexer,
        StandardNumberLexer,
        StandardDelimiterLexer,
        StandardRawStringLexer,
        StandardEscapedStringLexer,
        StandardShellLexer,
    >::new(inp.as_str())
    .into_iter()
    {
        println!("{tok_res:?}");
    }
}
