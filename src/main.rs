fn main() {
    for tok_res in
        kemsh::lexer::Lexer::<&str, &str, &str, &str>::new("###\"abc\"wda\"###").into_iter()
    {
        println!("{tok_res:?}");
    }
}
