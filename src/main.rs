mod lexer;

fn main() {
    let mut lexer_struct = lexer::Lexer::new("###\"abc\"wda\"###");
    while let Some(tok_res) = lexer_struct.lex::<&str, &str, &str, &str>() {
        println!("{tok_res:?}");
    }
}
