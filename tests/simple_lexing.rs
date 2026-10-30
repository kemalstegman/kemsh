// use std::convert::Infallible;
//
// use kemsh::lexer::*;
//
// fn lex_vec(
//     lexable: &str,
// ) -> Vec<
//     Result<
//         Token<'_, &str, &str, &str, &str>,
//         LexerError<Infallible, Infallible, Infallible, Infallible>,
//     >,
// > {
//     Lexer::<EscapedStringLexer, LexemeLexer, DelimiterLexer, ShellLexer>::new(lexable)
//         .into_iter()
//         .collect()
// }
//
// #[test]
// fn raw_string_00() {
//     assert_eq!(
//         lex_vec(r####"###"abc"wda"###"####),
//         vec![Ok(Token::RawString(r#"abc"wda"#)),]
//     );
// }
// #[test]
// fn raw_string_01() {
//     assert_eq!(
//         lex_vec(r####""abcwda""####),
//         vec![Ok(Token::RawString(r#"abcwda"#)),]
//     );
// }
//
// #[test]
// fn numbers_00() {
//     assert_eq!(
//         lex_vec("123 0x45 0b101 123u64 123i64"),
//         vec![
//             Ok(Token::SInteger(123)),
//             Ok(Token::UInteger(0x45)),
//             Ok(Token::UInteger(0b101)),
//             Ok(Token::UInteger(123)),
//             Ok(Token::SInteger(123)),
//         ]
//     );
// }
//
// #[test]
// fn delimiters_00() {
//     assert_eq!(
//         lex_vec("; [] ; { }"),
//         vec![
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Delimiter("[")),
//             Ok(Token::Delimiter("]")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Delimiter("{")),
//             Ok(Token::Delimiter("}")),
//         ]
//     );
// }
//
// #[test]
// fn lexemes_00() {
//     assert_eq!(
//         lex_vec("abc def"),
//         vec![Ok(Token::Lexeme("abc")), Ok(Token::Lexeme("def")),]
//     );
// }
//
// #[test]
// fn colon_00() {
//     assert_eq!(
//         lex_vec("abc : def"),
//         vec![
//             Ok(Token::Lexeme("abc")),
//             Ok(Token::Delimiter(":")),
//             Ok(Token::Lexeme("def")),
//         ]
//     );
// }
//
// #[test]
// fn colon_01() {
//     assert_eq!(
//         lex_vec("abc; : def"),
//         vec![
//             Ok(Token::Lexeme("abc")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Shell(" def")),
//         ]
//     );
// }
//
// #[test]
// fn colon_02() {
//     assert_eq!(
//         lex_vec("abc; :def;"),
//         vec![
//             Ok(Token::Lexeme("abc")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Shell("def")),
//             Ok(Token::Delimiter(";")),
//         ]
//     );
// }
//
// #[test]
// fn floats_00() {
//     assert_eq!(
//         lex_vec("123.45 0.5"),
//         vec![Ok(Token::Float(123.45)), Ok(Token::Float(0.5)),]
//     );
// }
//
// #[test]
// fn floats_01() {
//     assert_eq!(
//         lex_vec("123."),
//         vec![Ok(Token::SInteger(123)), Ok(Token::Delimiter(".")),]
//     );
// }
//
// #[test]
// fn zero_suffixes_00() {
//     assert_eq!(
//         lex_vec("0 0i64 0u64"),
//         vec![
//             Ok(Token::SInteger(0)),
//             Ok(Token::SInteger(0)),
//             Ok(Token::UInteger(0)),
//         ]
//     );
// }
//
// #[test]
// fn range_00() {
//     assert_eq!(
//         lex_vec("12..15"),
//         vec![
//             Ok(Token::SInteger(12)),
//             Ok(Token::Delimiter("..")),
//             Ok(Token::SInteger(15)),
//         ]
//     );
// }
//
// #[test]
// fn incomplete_suffix_00() {
//     let res_i = lex_vec("123i");
//     assert!(res_i[0].is_err());
//
//     let res_u = lex_vec("456u");
//     assert!(res_u[0].is_err());
// }
//
// #[test]
// fn malformed_base_numbers_00() {
//     let res_hex = lex_vec("0xG");
//     assert!(res_hex[0].is_err());
//
//     let res_bin = lex_vec("0b2");
//     assert!(res_bin[0].is_err());
// }
//
// #[test]
// fn consecutive_tokens_00() {
//     assert_eq!(
//         lex_vec("let x: i64 = 42..50;"),
//         vec![
//             Ok(Token::Lexeme("let")),
//             Ok(Token::Lexeme("x")),
//             Ok(Token::Delimiter(":")),
//             Ok(Token::Lexeme("i64")),
//             Ok(Token::Delimiter("=")),
//             Ok(Token::SInteger(42)),
//             Ok(Token::Delimiter("..")),
//             Ok(Token::SInteger(50)),
//             Ok(Token::Delimiter(";")),
//         ]
//     );
// }
//
// #[test]
// fn shell_eof_00() {
//     assert_eq!(
//         lex_vec("run; :echo hello world"),
//         vec![
//             Ok(Token::Lexeme("run")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Shell("echo hello world")),
//         ]
//     );
// }
//
// #[test]
// fn shell_multiple_00() {
//     assert_eq!(
//         lex_vec("task1; :cmd_one; task2; :cmd_two;"),
//         vec![
//             Ok(Token::Lexeme("task1")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Shell("cmd_one")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Lexeme("task2")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Shell("cmd_two")),
//             Ok(Token::Delimiter(";")),
//         ]
//     );
// }
//
// #[test]
// fn shell_whitespace_00() {
//     assert_eq!(
//         lex_vec("foo; :   spaced_command   ;"),
//         vec![
//             Ok(Token::Lexeme("foo")),
//             Ok(Token::Delimiter(";")),
//             Ok(Token::Shell("   spaced_command   ")),
//             Ok(Token::Delimiter(";")),
//         ]
//     );
// }
//
// #[test]
// fn test_punctuation_unclumping_00() {
//     assert_eq!(
//         lex_vec("+-*"),
//         vec![
//             Ok(Token::Delimiter("+")),
//             Ok(Token::Delimiter("-")),
//             Ok(Token::Delimiter("*")),
//         ]
//     );
// }
//
// #[test]
// fn test_multichar_delimiter_splitting_00() {
//     assert_eq!(
//         lex_vec("..="),
//         vec![Ok(Token::Delimiter("..")), Ok(Token::Delimiter("=")),]
//     );
// }
