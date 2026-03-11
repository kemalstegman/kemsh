use std::iter::Peekable;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    String(String),
    Number(i64),
    Delimeter(Delimeter),
}

#[derive(Debug)]
pub enum Delimeter {
    Whitespace,
    ExclamationMark,    // !
    Carret,             // ^
    Ampersand,          // &
    AmpersandAmpersand, // &&
    Asterisk,           // *
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    Minus,              // -
    Plus,               // +
    Equal,              // =
    EqualEqual,         // ==
    Pipe,               // |
    PipePipe,           // ||
    OpenBracket,        // [
    CloseBracket,       // ]
    OpenBrace,          // {
    CloseBrace,         // }
    Semicolon,          // ;
    Colon,              // :
    OpenAngleBracket,   // <
    CloseAngleBracket,  // >
    Comma,              // ,
    Period,             // .
    ForwardSlash,       // /
}

pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    iter: Peekable<I>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Option<Token>, LexError>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(lex_token(self.iter.next()?, &mut self.iter))
    }
}

#[derive(Debug)]
pub struct LexError {
    message: String,
}

fn lex_token(
    ch: char,
    iter: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Option<Token>, LexError> {
    match ch {
        ' ' | '\t' | '\n' | '\r' => {
            while let Some(_) = iter.next_if(|ch| match ch {
                ' ' | '\t' | '\n' | '\r' => true,
                _ => false,
            }) {}
            Ok(Some(Token::Delimeter(Delimeter::Whitespace)))
        }
        '"' => Ok(lex_string(iter, 0, false).map(|string| Token::String(string))),
        '#' => {
            let hashtag_count = count_hashtags(iter) + 1;
            if iter.next_if_eq(&'"').is_none() {
                return Err(LexError {
                    message: String::from("Expected quotation mark"),
                });
            }
            Ok(lex_string(iter, hashtag_count, false).map(|string| Token::String(string)))
        }
        'e' if iter.next_if_eq(&'"').is_some() => {
            Ok(lex_string(iter, 0, true).map(|string| Token::String(string)))
        }
        'e' if iter.next_if_eq(&'#').is_some() => {
            let hashtag_count = count_hashtags(iter) + 1;
            if iter.next_if_eq(&'"').is_none() {
                return Err(LexError {
                    message: String::from("Expected quotation mark"),
                });
            }
            Ok(lex_string(iter, hashtag_count, true).map(|string| Token::String(string)))
        }
        ch @ '0'..='9' => {
            let number_string = String::from(ch);
            Ok(Some(Token::Number(lex_number(iter, number_string)?)))
        }
        ch @ ('a'..='z' | 'A'..='Z' | '_') => {
            let mut identifier = String::from(ch);
            lex_identifier(iter, &mut identifier);
            Ok(Some(Token::Identifier(identifier)))
        }
        '!' => Ok(Some(Token::Delimeter(Delimeter::ExclamationMark))),
        '^' => Ok(Some(Token::Delimeter(Delimeter::Carret))),
        '&' => match iter.next_if_eq(&'&') {
            Some(_) => Ok(Some(Token::Delimeter(Delimeter::AmpersandAmpersand))),
            None => Ok(Some(Token::Delimeter(Delimeter::Ampersand))),
        },
        '*' => Ok(Some(Token::Delimeter(Delimeter::Asterisk))),
        '(' => Ok(Some(Token::Delimeter(Delimeter::OpenParenthesis))),
        ')' => Ok(Some(Token::Delimeter(Delimeter::CloseParenthesis))),
        '-' => Ok(Some(Token::Delimeter(Delimeter::Minus))),
        '+' => Ok(Some(Token::Delimeter(Delimeter::Plus))),
        '=' => match iter.next_if_eq(&'=') {
            Some(_) => Ok(Some(Token::Delimeter(Delimeter::EqualEqual))),
            None => Ok(Some(Token::Delimeter(Delimeter::Equal))),
        },
        '|' => match iter.next_if_eq(&'|') {
            Some(_) => Ok(Some(Token::Delimeter(Delimeter::PipePipe))),
            None => Ok(Some(Token::Delimeter(Delimeter::Pipe))),
        },
        '[' => Ok(Some(Token::Delimeter(Delimeter::OpenBracket))),
        ']' => Ok(Some(Token::Delimeter(Delimeter::CloseBracket))),
        '{' => Ok(Some(Token::Delimeter(Delimeter::OpenBrace))),
        '}' => Ok(Some(Token::Delimeter(Delimeter::CloseBrace))),
        ';' => Ok(Some(Token::Delimeter(Delimeter::Semicolon))),
        ':' => Ok(Some(Token::Delimeter(Delimeter::Colon))),
        '<' => Ok(Some(Token::Delimeter(Delimeter::OpenAngleBracket))),
        '>' => Ok(Some(Token::Delimeter(Delimeter::CloseAngleBracket))),
        ',' => Ok(Some(Token::Delimeter(Delimeter::Comma))),
        '.' => Ok(Some(Token::Delimeter(Delimeter::Period))),
        '/' => Ok(Some(Token::Delimeter(Delimeter::ForwardSlash))),
        ch => Err(LexError {
            message: format!("Unexpected character: {:?}", ch),
        }),
    }
}

fn lex_identifier(iter: &mut Peekable<impl Iterator<Item = char>>, string: &mut String) {
    while let Some(ch) = iter.next_if(|ch| match ch {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }) {
        string.push(ch);
    }
}

fn lex_number(
    iter: &mut Peekable<impl Iterator<Item = char>>,
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

fn count_hashtags(iter: &mut Peekable<impl Iterator<Item = char>>) -> u32 {
    let mut hashtag_count = 1;
    while let Some(_) = iter.next_if_eq(&'#') {
        hashtag_count += 1;
    }
    hashtag_count
}

fn lex_string(
    iter: &mut Peekable<impl Iterator<Item = char>>,
    hashtag_delimeter_count: u32,
    _escaped: bool,
) -> Option<String> {
    let mut string = String::new();
    'charpush: loop {
        match iter.next()? {
            // '\\' if escaped => todo!(),
            '"' => {
                for i in 0..hashtag_delimeter_count {
                    match iter.next()? {
                        '#' => (),
                        ch => {
                            string.push('"');
                            for _ in 0..i {
                                string.push('#');
                            }
                            string.push(ch);
                            continue 'charpush;
                        }
                    }
                }
                return Some(string);
            }
            ch => string.push(ch),
        }
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
