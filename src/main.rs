mod lexer;

fn main() {
    let mut lexer_struct = lexer::Lexer::new("let x: f64 = 6.03; :ls;");
    while let Some(tok_res) = lexer_struct.lex::<&str, &str, &str, &str>() {
        println!("{tok_res:?}");
    }
}
