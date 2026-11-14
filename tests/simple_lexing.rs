use std::convert::Infallible;

use kemsh::lexer::*;

fn lex_vec(
    lexable: &str,
) -> Vec<
    Result<
        Token<Lexeme<'_>, Number<'_>, Delimiter, &str, Infallible, &str>,
        LexerError<FatalLexerError<Infallible, Infallible, char, char, Infallible, Infallible>>,
    >,
> {
    Lexer::new(lexable).into_iter().collect()
}

#[test]
fn raw_string_00() {
    assert_eq!(
        lex_vec(r####"###"abc"wda"###"####),
        vec![Ok(Token::RawString(r#"abc"wda"#)),]
    );
}
#[test]
fn raw_string_01() {
    assert_eq!(
        lex_vec(r####""abcwda""####),
        vec![Ok(Token::RawString(r#"abcwda"#)),]
    );
}

#[test]
fn numbers_00() {
    assert_eq!(
        lex_vec("123 0x45 0b101 123u64 123i64"),
        vec![
            Ok(Token::Number(Number::Integer("123"))),
            Ok(Token::Number(Number::Hex("45"))),
            Ok(Token::Number(Number::Binary("101"))),
            Ok(Token::Number(Number::Unsigned("123"))),
            Ok(Token::Number(Number::Signed("123"))),
        ]
    );
}

#[test]
fn delimiters_00() {
    assert_eq!(
        lex_vec("; [] ; { }"),
        vec![
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Delimiter(Delimiter::OpenSquareBracket)),
            Ok(Token::Delimiter(Delimiter::CloseSquareBracket)),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Delimiter(Delimiter::OpenBrace)),
            Ok(Token::Delimiter(Delimiter::CloseBrace)),
        ]
    );
}

#[test]
fn lexemes_00() {
    assert_eq!(
        lex_vec("abc def"),
        vec![
            Ok(Token::Lexeme(Lexeme::Identifier("abc"))),
            Ok(Token::Lexeme(Lexeme::Identifier("def"))),
        ]
    );
}

#[test]
fn colon_00() {
    assert_eq!(
        lex_vec("abc : def"),
        vec![
            Ok(Token::Lexeme(Lexeme::Identifier("abc"))),
            Ok(Token::Delimiter(Delimiter::Colon)),
            Ok(Token::Lexeme(Lexeme::Identifier("def"))),
        ]
    );
}

#[test]
fn colon_01() {
    assert_eq!(
        lex_vec("abc; : def"),
        vec![
            Ok(Token::Lexeme(Lexeme::Identifier("abc"))),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Shell(" def")),
        ]
    );
}

#[test]
fn colon_02() {
    assert_eq!(
        lex_vec("abc; :def;"),
        vec![
            Ok(Token::Lexeme(Lexeme::Identifier("abc"))),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Shell("def")),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
        ]
    );
}

#[test]
fn floats_00() {
    assert_eq!(
        lex_vec("123.45 0.5"),
        vec![
            Ok(Token::Number(Number::Float("123.45"))),
            Ok(Token::Number(Number::Float("0.5"))),
        ]
    );
}

#[test]
fn floats_01() {
    assert_eq!(
        lex_vec("123."),
        vec![
            Ok(Token::Number(Number::Integer("123"))),
            Ok(Token::Delimiter(Delimiter::Period)),
        ]
    );
}

#[test]
fn zero_suffixes_00() {
    assert_eq!(
        lex_vec("0 0i64 0u64"),
        vec![
            Ok(Token::Number(Number::Integer("0"))),
            Ok(Token::Number(Number::Signed("0"))),
            Ok(Token::Number(Number::Unsigned("0"))),
        ]
    );
}

// #[test]
// fn range_00() {
//     assert_eq!(
//         lex_vec("12..15"),
//         vec![
//             Ok(Token::Number(Number::Integer("12"))),
//             Ok(Token::Delimiter("..")),
//             Ok(Token::Number(Number::Integer("15"))),
//         ]
//     );
// }

#[test]
fn incomplete_suffix_00() {
    assert_eq!(
        lex_vec("123i"),
        vec![
            Ok(Token::Number(Number::Integer("123"))),
            Ok(Token::Lexeme(Lexeme::Identifier("i"))),
        ]
    );
    assert_eq!(
        lex_vec("456u"),
        vec![
            Ok(Token::Number(Number::Integer("456"))),
            Ok(Token::Lexeme(Lexeme::Identifier("u"))),
        ]
    );
}

#[test]
fn malformed_base_numbers_00() {
    assert_eq!(
        lex_vec("0xG"),
        vec![
            Ok(Token::Number(Number::Hex(""))),
            Ok(Token::Lexeme(Lexeme::Identifier("G"))),
        ]
    );
    assert_eq!(
        lex_vec("0b2"),
        vec![
            Ok(Token::Number(Number::Binary(""))),
            Ok(Token::Number(Number::Integer("2"))),
        ]
    );
}

// #[test]
// fn consecutive_tokens_00() {
//     assert_eq!(
//         lex_vec("let x: i64 = 42..50;"),
//         vec![
//             Ok(Token::Lexeme(Lexeme::LexLet)),
//             Ok(Token::Lexeme(Lexeme::Identifier("x"))),
//             Ok(Token::Delimiter(Delimiter::Colon)),
//             Ok(Token::Lexeme(Lexeme::LexI64)),
//             Ok(Token::Delimiter(Delimiter::Equal)),
//             Ok(Token::Number(Number::Integer("42"))),
//             Ok(Token::Delimiter("..")),
//             Ok(Token::Number(Number::Integer("50"))),
//             Ok(Token::Delimiter(Delimiter::Semicolon)),
//         ]
//     );
// }

#[test]
fn shell_eof_00() {
    assert_eq!(
        lex_vec("run; :echo hello world"),
        vec![
            Ok(Token::Lexeme(Lexeme::LexRun)),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Shell("echo hello world")),
        ]
    );
}

#[test]
fn shell_multiple_00() {
    assert_eq!(
        lex_vec("task1; :cmd_one; task2; :cmd_two;"),
        vec![
            Ok(Token::Lexeme(Lexeme::Identifier("task1"))),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Shell("cmd_one")),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Lexeme(Lexeme::Identifier("task2"))),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Shell("cmd_two")),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
        ]
    );
}

#[test]
fn shell_whitespace_00() {
    assert_eq!(
        lex_vec("foo; :   spaced_command   ;"),
        vec![
            Ok(Token::Lexeme(Lexeme::Identifier("foo"))),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
            Ok(Token::Shell("   spaced_command   ")),
            Ok(Token::Delimiter(Delimiter::Semicolon)),
        ]
    );
}

#[test]
fn test_punctuation_unclumping_00() {
    assert_eq!(
        lex_vec("+-*"),
        vec![
            Ok(Token::Delimiter(Delimiter::Plus)),
            Ok(Token::Delimiter(Delimiter::HyphenMinus)),
            Ok(Token::Delimiter(Delimiter::Asterisk)),
        ]
    );
}

// #[test]
// fn test_multichar_delimiter_splitting_00() {
//     assert_eq!(
//         lex_vec("..="),
//         vec![Ok(Token::Delimiter("..")), Ok(Token::Delimiter("=")),]
//     );
// }
