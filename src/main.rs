use std::io::Read;

fn main() {
    let mut inp = String::new();
    std::io::stdin().lock().read_to_string(&mut inp).unwrap();
    for tok_res in kemsh::lexer::Lexer::<&str, &str, &str, &str>::new(inp.as_str()).into_iter() {
        println!("{tok_res:?}");
    }
}
