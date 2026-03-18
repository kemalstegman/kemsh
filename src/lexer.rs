mod token;
pub use token::{Delimeter, Token};

pub use crate::char_buffer::{LexableParsableCharBuffer, TrackingPeekable};

#[derive(Debug)]
pub struct LexError {
    message: String,
}

pub fn lex(
    char_buffer: &mut impl LexableParsableCharBuffer,
) -> Option<Result<Option<Token>, LexError>> {
    let (res, consumed) = {
        let mut iter = TrackingPeekable::new(char_buffer.chars());
        (lex_token(&mut iter), iter.next_consumed())
    };
    if let Some(Ok(Some(_))) = res {
        char_buffer.lex_consume(consumed);
    }
    res
}

fn lex_token(
    iter: &mut TrackingPeekable<impl Iterator<Item = char>>,
) -> Option<Result<Option<Token>, LexError>> {
    match iter.next()? {
        ' ' | '\t' | '\n' | '\r' => {
            while let Some(_) = iter.next_if(|ch| match ch {
                ' ' | '\t' | '\n' | '\r' => true,
                _ => false,
            }) {}
            Some(Ok(Some(Token::Delimeter(Delimeter::Whitespace))))
        }
        '"' => Some(lex_string(iter, 0, false).map(|opt| opt.map(|string| Token::String(string)))),
        '#' => {
            let delimeter_hashtags = match consume_string_start_delimeter(iter) {
                Ok(n) => n + 1,
                Err(e) => return Some(Err(e)),
            };
            Some(
                lex_string(iter, delimeter_hashtags, false)
                    .map(|opt| opt.map(|string| Token::String(string))),
            )
        }
        'e' if iter.next_if_eq(&'"').is_some() => {
            Some(lex_string(iter, 0, true).map(|opt| opt.map(|string| Token::String(string))))
        }
        'e' if iter.next_if_eq(&'#').is_some() => {
            let delimeter_hashtags = match consume_string_start_delimeter(iter) {
                Ok(n) => n + 1,
                Err(e) => return Some(Err(e)),
            };
            Some(
                lex_string(iter, delimeter_hashtags, true)
                    .map(|opt| opt.map(|string| Token::String(string))),
            )
        }
        ch @ '0'..='9' => {
            let number_string = String::from(ch);
            Some(lex_number(iter, number_string).map(|n| Some(Token::Number(n))))
        }
        ch @ ('a'..='z' | 'A'..='Z' | '_') => {
            let mut identifier = String::from(ch);
            lex_identifier(iter, &mut identifier);
            Some(Ok(Some(Token::Identifier(identifier))))
        }
        '!' => Some(Ok(Some(Token::Delimeter(Delimeter::ExclamationMark)))),
        '^' => Some(Ok(Some(Token::Delimeter(Delimeter::Carret)))),
        '&' => match iter.next_if_eq(&'&') {
            Some(_) => Some(Ok(Some(Token::Delimeter(Delimeter::AmpersandAmpersand)))),
            None => Some(Ok(Some(Token::Delimeter(Delimeter::Ampersand)))),
        },
        '*' => Some(Ok(Some(Token::Delimeter(Delimeter::Asterisk)))),
        '(' => Some(Ok(Some(Token::Delimeter(Delimeter::OpenParenthesis)))),
        ')' => Some(Ok(Some(Token::Delimeter(Delimeter::CloseParenthesis)))),
        '-' => Some(Ok(Some(Token::Delimeter(Delimeter::Minus)))),
        '+' => Some(Ok(Some(Token::Delimeter(Delimeter::Plus)))),
        '=' => match iter.next_if_eq(&'=') {
            Some(_) => Some(Ok(Some(Token::Delimeter(Delimeter::EqualEqual)))),
            None => Some(Ok(Some(Token::Delimeter(Delimeter::Equal)))),
        },
        '|' => match iter.next_if_eq(&'|') {
            Some(_) => Some(Ok(Some(Token::Delimeter(Delimeter::PipePipe)))),
            None => Some(Ok(Some(Token::Delimeter(Delimeter::Pipe)))),
        },
        '[' => Some(Ok(Some(Token::Delimeter(Delimeter::OpenBracket)))),
        ']' => Some(Ok(Some(Token::Delimeter(Delimeter::CloseBracket)))),
        '{' => Some(Ok(Some(Token::Delimeter(Delimeter::OpenBrace)))),
        '}' => Some(Ok(Some(Token::Delimeter(Delimeter::CloseBrace)))),
        ';' => Some(Ok(Some(Token::Delimeter(Delimeter::Semicolon)))),
        ':' => Some(Ok(Some(Token::Delimeter(Delimeter::Colon)))),
        '<' => Some(Ok(Some(Token::Delimeter(Delimeter::OpenAngleBracket)))),
        '>' => Some(Ok(Some(Token::Delimeter(Delimeter::CloseAngleBracket)))),
        ',' => Some(Ok(Some(Token::Delimeter(Delimeter::Comma)))),
        '.' => Some(Ok(Some(Token::Delimeter(Delimeter::Period)))),
        '/' => Some(Ok(Some(Token::Delimeter(Delimeter::ForwardSlash)))),
        ch => Some(Err(LexError {
            message: format!("Unexpected character: {:?}", ch),
        })),
    }
}

/// consumes consecutive `#` and then one `"`. success if the `"` was consumed
/// and returns the number of `#` consumed.
fn consume_string_start_delimeter(
    iter: &mut TrackingPeekable<impl Iterator<Item = char>>,
) -> Result<usize, LexError> {
    let mut hashtag_count = 0;
    while iter.next_if_eq(&'#').is_some() {
        hashtag_count += 1;
    }
    if iter.next_if_eq(&'"').is_some() {
        Ok(hashtag_count)
    } else {
        Err(LexError {
            message: String::from("Expected quotation mark"),
        })
    }
}

fn lex_string(
    iter: &mut TrackingPeekable<impl Iterator<Item = char>>,
    delimeter_hashtags: usize,
    _escaped: bool, // todo!()
) -> Result<Option<String>, LexError> {
    let mut string = String::new();
    'charpush: loop {
        match iter.next() {
            None => return Ok(None),
            // '\\' if escaped => todo!(),
            Some('"') => {
                for i in 0..delimeter_hashtags {
                    match iter.next() {
                        None => return Ok(None),
                        Some('#') => (),
                        Some(ch) => {
                            string.push('"');
                            for _ in 0..i {
                                string.push('#');
                            }
                            string.push(ch);
                            continue 'charpush;
                        }
                    }
                }
                return Ok(Some(string));
            }
            Some(ch) => string.push(ch),
        }
    }
}

fn lex_identifier(iter: &mut TrackingPeekable<impl Iterator<Item = char>>, string: &mut String) {
    while let Some(ch) = iter.next_if(|ch| match ch {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }) {
        string.push(ch);
    }
}

fn lex_number(
    iter: &mut TrackingPeekable<impl Iterator<Item = char>>,
    mut number_string: String,
) -> Result<i64, LexError> {
    while let Some(ch) = iter.next_if(|ch| match ch {
        '0'..='9' => true,
        _ => false,
    }) {
        number_string.push(ch);
    }
    match number_string.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(LexError {
            message: format!("Invalid (i64) number: {:?}", number_string),
        }),
    }
}

// "let" => Token::Keyword(TokenKeyword::Let),
// "for" => Token::Keyword(TokenKeyword::For),
// "while" => Token::Keyword(TokenKeyword::While),
// "loop" => Token::Keyword(TokenKeyword::Loop),
// "return" => Token::Keyword(TokenKeyword::Return),
// "break" => Token::Keyword(TokenKeyword::Break),
// "run" => Token::Keyword(TokenKeyword::Run),
// "spawn" => Token::Keyword(TokenKeyword::Spawn),
// "echo" => Token::Keyword(TokenKeyword::Echo),
// "true" => Token::Boolean(true),
// "false" => Token::Boolean(false),
// _ => Token::VariableName(identifier),
