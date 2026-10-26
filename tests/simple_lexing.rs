use kemsh::lexer::*;

fn lex_vec(
    lexable: &str,
) -> Vec<Result<Token<'_, &str, &str, &str, &str>, LexerError<'_, &str, &str, &str, &str>>> {
    Lexer::<&str, &str, &str, &str>::new(lexable)
        .into_iter()
        .collect()
}

#[test]
fn test_raw_string_00() {
    assert_eq!(
        lex_vec(r####"###"abc"wda"###"####),
        vec![Ok(Token::RawString(r#"abc"wda"#)),]
    );
}
#[test]
fn test_raw_string_01() {
    assert_eq!(
        lex_vec(r####""abcwda""####),
        vec![Ok(Token::RawString(r#"abcwda"#)),]
    );
}

#[test]
fn test_numbers_00() {
    assert_eq!(
        lex_vec("123 0x45 0b101 123u64 123i64"),
        vec![
            Ok(Token::SInteger(123)),
            Ok(Token::UInteger(0x45)),
            Ok(Token::UInteger(0b101)),
            Ok(Token::UInteger(123)),
            Ok(Token::SInteger(123)),
        ]
    );
}

#[test]
fn test_delimiters_00() {
    assert_eq!(
        lex_vec("; [] ; { }"),
        vec![
            Ok(Token::Delimiter(";")),
            Ok(Token::Delimiter("[]")),
            Ok(Token::Delimiter(";")),
            Ok(Token::Delimiter("{")),
            Ok(Token::Delimiter("}")),
        ]
    );
}

#[test]
fn test_lexemes_00() {
    assert_eq!(
        lex_vec("abc def"),
        vec![Ok(Token::Lexeme("abc")), Ok(Token::Lexeme("def")),]
    );
}

#[test]
fn test_colon_00() {
    assert_eq!(
        lex_vec("abc : def"),
        vec![
            Ok(Token::Lexeme("abc")),
            Ok(Token::Delimiter(":")),
            Ok(Token::Lexeme("def")),
        ]
    );
}

#[test]
fn test_colon_01() {
    assert_eq!(
        lex_vec("abc; : def"),
        vec![
            Ok(Token::Lexeme("abc")),
            Ok(Token::Delimiter(";")),
            Ok(Token::Shell(" def")),
        ]
    );
}

#[test]
fn test_colon_02() {
    assert_eq!(
        lex_vec("abc; :def;"),
        vec![
            Ok(Token::Lexeme("abc")),
            Ok(Token::Delimiter(";")),
            Ok(Token::Shell("def")),
            Ok(Token::Delimiter(";")),
        ]
    );
}

#[test]
fn test_floats_00() {
    assert_eq!(
        lex_vec("123.45 0.5"),
        vec![Ok(Token::Float(123.45)), Ok(Token::Float(0.5)),]
    );
}

#[test]
fn test_float_error_00() {
    assert!(lex_vec("123.")[0].is_err());
}

#[test]
fn test_zero_suffixes_00() {
    assert_eq!(
        lex_vec("0 0i64 0u64"),
        vec![
            Ok(Token::SInteger(0)),
            Ok(Token::SInteger(0)),
            Ok(Token::UInteger(0)),
        ]
    );
}
