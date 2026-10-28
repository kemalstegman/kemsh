use std::io::Read;

use kemsh::lexer::{DelimiterLexer, EscapedStringLexer, LexemeLexer, ShellLexer};

fn main() {
    let mut inp = String::new();
    std::io::stdin().lock().read_to_string(&mut inp).unwrap();
    for tok_res in
        kemsh::lexer::Lexer::<EscapedStringLexer, LexemeLexer, DelimiterLexer, ShellLexer>::new(
            inp.as_str(),
        )
        .into_iter()
    {
        println!("{tok_res:?}");
    }
}
