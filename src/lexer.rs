use std::iter::Peekable;

mod token;
pub use token::{Token, TokenDelimeter, TokenKeyword};

pub struct LexerIter<I: Iterator<Item = Result<char, ()>>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = Result<char, ()>>> Iterator for LexerIter<I> {
    type Item = Result<Token, LexError>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<I: Iterator<Item = Result<char, ()>>> LexerIter<I> {
    pub fn new(iter: I) -> Self {
        Self { iter: iter.peekable() }
    }
}

#[derive(Debug)]
pub enum LexError {
    Char(()),
    Lex(FatalLexError),
    Incomplete,
}

#[derive(Debug)]
pub struct FatalLexError {
    message: String,
}

pub struct Lexer<I>
where
    I: Iterator<Item = Result<char, ()>>,
{
    iter: Peekable<I>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = Result<char, ()>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
    fn
    fn lex(&mut self) -> Option<Result<Token, LexError>> {
        while let Some(_) = self.iter.next_if(|ch| match ch {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }) {}
        match match self.iter.next()? {
            Ok(c) => c,
            Err(err) => return Some(Err(LexError::Char(err))),
        } {
            // ' ' | '\t' | '\n' | '\r' => {
            //     while let Some(_) = self.iter.next_if(|ch| match ch {
            //         ' ' | '\t' | '\n' | '\r' => true,
            //         _ => false,
            //     }) {}
            //     Some(Ok(Some(Token::Delimeter(TokenDelimeter::Whitespace))))
            // }
            '"' => Some(
                lex_string(&mut self.iter, 0, false)
                    .map(|opt| opt.map(|string| Token::String(string))),
            ),
            '#' => {
                let delimeter_hashtags = match consume_string_start_delimeter(&mut self.iter) {
                    Ok(n) => n + 1,
                    Err(e) => return Some(Err(e)),
                };
                Some(
                    lex_string(&mut self.iter, delimeter_hashtags, false)
                        .map(|opt| opt.map(|string| Token::String(string))),
                )
            }
            'e' if self.iter.next_if_eq(&'"').is_some() => Some(
                lex_string(&mut self.iter, 0, true)
                    .map(|opt| opt.map(|string| Token::String(string))),
            ),
            'e' if self.iter.next_if_eq(&'#').is_some() => {
                let delimeter_hashtags = match consume_string_start_delimeter(&mut self.iter) {
                    Ok(n) => n + 1,
                    Err(e) => return Some(Err(e)),
                };
                Some(
                    lex_string(&mut self.iter, delimeter_hashtags, true)
                        .map(|opt| opt.map(|string| Token::String(string))),
                )
            }
            ch @ '0'..='9' => {
                let number_string = String::from(ch);
                Some(lex_number(&mut self.iter, number_string).map(|n| Some(Token::Number(n))))
            }
            ch @ ('a'..='z' | 'A'..='Z' | '_') => {
                let mut lexeme = String::from(ch);
                lex_lexeme(&mut self.iter, &mut lexeme);
                Some(Ok(Some(Token::from_lexeme(lexeme))))
            }
            '!' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::ExclamationMark)))),
            '^' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Carret)))),
            '&' => match self.iter.next_if_eq(&'&') {
                Some(_) => Some(Ok(Some(Token::Delimeter(
                    TokenDelimeter::AmpersandAmpersand,
                )))),
                None => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Ampersand)))),
            },
            '*' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Asterisk)))),
            '(' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenParenthesis)))),
            ')' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::CloseParenthesis)))),
            '-' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Minus)))),
            '+' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Plus)))),
            '=' => match self.iter.next_if_eq(&'=') {
                Some(_) => Some(Ok(Some(Token::Delimeter(TokenDelimeter::EqualEqual)))),
                None => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Equal)))),
            },
            '|' => match self.iter.next_if_eq(&'|') {
                Some(_) => Some(Ok(Some(Token::Delimeter(TokenDelimeter::PipePipe)))),
                None => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Pipe)))),
            },
            '[' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenBracket)))),
            ']' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::CloseBracket)))),
            '{' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenBrace)))),
            '}' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::CloseBrace)))),
            ';' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Semicolon)))),
            ':' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Colon)))),
            '<' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenAngleBracket)))),
            '>' => Some(Ok(Some(Token::Delimeter(
                TokenDelimeter::CloseAngleBracket,
            )))),
            ',' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Comma)))),
            '.' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Period)))),
            '/' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::ForwardSlash)))),
            ch => Some(Err(LexError {
                message: format!("Unexpected character: {:?}", ch),
            })),
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = Result<char, ()>>,
{
    type Item = Result<Token, LexError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

// #[derive(Debug)]
// pub struct LexError {
//     message: String,
// }

// pub struct Lexer<I: Iterator<Item = char>> {
//     iter: Peekable<I>,
// }

// impl<I: Iterator<Item = char>> Lexer<I> {
//     pub fn new(iter: I) -> Self {
//         Self {
//             iter: iter.peekable(),
//         }
//     }
//     pub fn lex_token(&mut self) -> Option<Result<Option<Token>, LexError>> {
//         while let Some(_) = self.iter.next_if(|ch| match ch {
//             ' ' | '\t' | '\n' | '\r' => true,
//             _ => false,
//         }) {}
//         match self.iter.next()? {
//             // ' ' | '\t' | '\n' | '\r' => {
//             //     while let Some(_) = self.iter.next_if(|ch| match ch {
//             //         ' ' | '\t' | '\n' | '\r' => true,
//             //         _ => false,
//             //     }) {}
//             //     Some(Ok(Some(Token::Delimeter(TokenDelimeter::Whitespace))))
//             // }
//             '"' => Some(
//                 lex_string(&mut self.iter, 0, false)
//                     .map(|opt| opt.map(|string| Token::String(string))),
//             ),
//             '#' => {
//                 let delimeter_hashtags = match consume_string_start_delimeter(&mut self.iter) {
//                     Ok(n) => n + 1,
//                     Err(e) => return Some(Err(e)),
//                 };
//                 Some(
//                     lex_string(&mut self.iter, delimeter_hashtags, false)
//                         .map(|opt| opt.map(|string| Token::String(string))),
//                 )
//             }
//             'e' if self.iter.next_if_eq(&'"').is_some() => Some(
//                 lex_string(&mut self.iter, 0, true)
//                     .map(|opt| opt.map(|string| Token::String(string))),
//             ),
//             'e' if self.iter.next_if_eq(&'#').is_some() => {
//                 let delimeter_hashtags = match consume_string_start_delimeter(&mut self.iter) {
//                     Ok(n) => n + 1,
//                     Err(e) => return Some(Err(e)),
//                 };
//                 Some(
//                     lex_string(&mut self.iter, delimeter_hashtags, true)
//                         .map(|opt| opt.map(|string| Token::String(string))),
//                 )
//             }
//             ch @ '0'..='9' => {
//                 let number_string = String::from(ch);
//                 Some(lex_number(&mut self.iter, number_string).map(|n| Some(Token::Number(n))))
//             }
//             ch @ ('a'..='z' | 'A'..='Z' | '_') => {
//                 let mut lexeme = String::from(ch);
//                 lex_lexeme(&mut self.iter, &mut lexeme);
//                 Some(Ok(Some(Token::from_lexeme(lexeme))))
//             }
//             '!' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::ExclamationMark)))),
//             '^' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Carret)))),
//             '&' => match self.iter.next_if_eq(&'&') {
//                 Some(_) => Some(Ok(Some(Token::Delimeter(
//                     TokenDelimeter::AmpersandAmpersand,
//                 )))),
//                 None => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Ampersand)))),
//             },
//             '*' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Asterisk)))),
//             '(' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenParenthesis)))),
//             ')' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::CloseParenthesis)))),
//             '-' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Minus)))),
//             '+' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Plus)))),
//             '=' => match self.iter.next_if_eq(&'=') {
//                 Some(_) => Some(Ok(Some(Token::Delimeter(TokenDelimeter::EqualEqual)))),
//                 None => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Equal)))),
//             },
//             '|' => match self.iter.next_if_eq(&'|') {
//                 Some(_) => Some(Ok(Some(Token::Delimeter(TokenDelimeter::PipePipe)))),
//                 None => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Pipe)))),
//             },
//             '[' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenBracket)))),
//             ']' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::CloseBracket)))),
//             '{' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenBrace)))),
//             '}' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::CloseBrace)))),
//             ';' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Semicolon)))),
//             ':' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Colon)))),
//             '<' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::OpenAngleBracket)))),
//             '>' => Some(Ok(Some(Token::Delimeter(
//                 TokenDelimeter::CloseAngleBracket,
//             )))),
//             ',' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Comma)))),
//             '.' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::Period)))),
//             '/' => Some(Ok(Some(Token::Delimeter(TokenDelimeter::ForwardSlash)))),
//             ch => Some(Err(LexError {
//                 message: format!("Unexpected character: {:?}", ch),
//             })),
//         }
//     }
// }

// impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
//     type Item = Result<Option<Token>, LexError>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.lex_token()
//     }
// }

// /// consumes consecutive `#` and then one `"`. success if the `"` was consumed
// /// and returns the number of `#` consumed.
// fn consume_string_start_delimeter(
//     iter: &mut Peekable<impl Iterator<Item = char>>,
// ) -> Result<usize, LexError> {
//     let mut hashtag_count = 0;
//     while iter.next_if_eq(&'#').is_some() {
//         hashtag_count += 1;
//     }
//     if iter.next_if_eq(&'"').is_some() {
//         Ok(hashtag_count)
//     } else {
//         Err(LexError {
//             message: String::from("Expected quotation mark"),
//         })
//     }
// }

// fn lex_string(
//     iter: &mut Peekable<impl Iterator<Item = char>>,
//     delimeter_hashtags: usize,
//     _escaped: bool, // todo!()
// ) -> Result<Option<String>, LexError> {
//     let mut string = String::new();
//     'charpush: loop {
//         match iter.next() {
//             None => return Ok(None),
//             // '\\' if escaped => todo!(),
//             Some('"') => {
//                 for i in 0..delimeter_hashtags {
//                     match iter.next() {
//                         None => return Ok(None),
//                         Some('#') => (),
//                         Some(ch) => {
//                             string.push('"');
//                             for _ in 0..i {
//                                 string.push('#');
//                             }
//                             string.push(ch);
//                             continue 'charpush;
//                         }
//                     }
//                 }
//                 return Ok(Some(string));
//             }
//             Some(ch) => string.push(ch),
//         }
//     }
// }

// fn lex_lexeme(iter: &mut Peekable<impl Iterator<Item = char>>, string: &mut String) {
//     while let Some(ch) = iter.next_if(|ch| match ch {
//         'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
//         _ => false,
//     }) {
//         string.push(ch);
//     }
// }

// fn lex_number(
//     iter: &mut Peekable<impl Iterator<Item = char>>,
//     mut number_string: String,
// ) -> Result<i64, LexError> {
//     while let Some(ch) = iter.next_if(|ch| match ch {
//         '0'..='9' => true,
//         _ => false,
//     }) {
//         number_string.push(ch);
//     }
//     match number_string.parse::<i64>() {
//         Ok(n) => Ok(n),
//         Err(_) => Err(LexError {
//             message: format!("Invalid (i64) number: {:?}", number_string),
//         }),
//     }
// }
