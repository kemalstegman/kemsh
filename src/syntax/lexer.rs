//! todo: hex and binary numbers, escaped strings, source errors
//!
//! The following is an incomplete abstraction of my lexer.
//!
//! There are a few categories of tokens the lexer will produce: lexemes, reserved and identifiers; symbol
//! delimeters; strings; and numbers. The lexer will necessitates having states; the most obvious example
//! would be parsing strings. Every character in between the delimeters of the string needs to be consumed
//! one-for-one, and the lexer cannot produce a token until the ending delimeter is seen, and thus seeing
//! the end of the stream of characters before this ending delimeter would cause a lexer error.
//!
//! # Behavior: Expecting
//! Basically there are three states the lexer can be in: (1) not consuming, where encountering the end of
//! the character stream means no token is produced, (2) consuming not expecting, where encountring the end
//! of the character stream will not create an error and a token can (and will) be produced, and (3) consuming
//! and expecting, where encountering the end of the character stream will immediately produce an incompletion
//! error. One caveat is that "consumption" in this context does not always mean the character is permanently
//! consumed; it could be only peeked at, and then consumed based on a condition, or not consumed at all.
//!
//! # General Lexing Behavior
//! Before parsing any tokens, *the lexer shall discard any whitespace characters*. This is the only spot
//! whitespace tokens should be lexed.
//!
//! When lexing lexemes, the lexer will always be in a "consuming not expecting" state. Every lexeme can be
//! abruptly converted into a token, though that doesn't always mean the parser will enjoy a lexeme directly
//! before the end of the character stream.
//!
//! Numbers will be in a "consuming not expecting" state with a couple of exceptions. If the two characters
//! match a valid prefix (e.g. '0x' and '0b'), the next character will be consumed in a "consuming and expecting"
//! state. Similarly, when a floating point number consumes a '.', the next character will be consumed in a
//! "consuming and expecting" state.
//!
//! Symbol delimeters will always be in a "consuming not expecting" state.
//!
//! Strings will be in a "consuming and expecting" state.

pub mod token;

use std::marker::PhantomData;

use crate::{
    abstract_lookahead::ErrorBubbledNLookahead,
    syntax::lexer::token::{Delimeter, Lexeme, Token},
};

pub struct Lexer<I, E>
where
    I: ErrorBubbledNLookahead<2, char, LexerError<E>>,
{
    iter: I,
    _marker: PhantomData<E>,
}

impl<I, E> Lexer<I, E>
where
    I: ErrorBubbledNLookahead<2, char, LexerError<E>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            _marker: PhantomData,
        }
    }
    pub fn lex(&mut self) -> Option<Result<Token, LexerError<E>>> {
        loop {
            match self.iter.next()? {
                Err(err) => return Some(Err(err)),
                Ok(ch) if ch.is_ascii_whitespace() => continue,
                Ok(ch) => return Some(self.lex_token(ch)),
            }
        }
    }
    fn lex_token(&mut self, ch: char) -> Result<Token, LexerError<E>> {
        match ch {
            '"' | '#' => Ok(Token::LiteralString(self.lex_literal_string(ch)?)),
            '0'..='9' => match self.lex_number(ch)? {
                (bytes, false) => Ok(Token::Integer(i64::from_ne_bytes(bytes))),
                (bytes, true) => Ok(Token::Float(f64::from_ne_bytes(bytes))),
            },
            'a'..='z' | 'A'..='Z' | '_' => Ok(Token::from_lexeme(self.lex_lexeme(ch)?)),
            '*' => Ok(Token::Delimeter(Delimeter::Asterisk)),
            '-' => Ok(Token::Delimeter(Delimeter::Minus)),
            '+' => Ok(Token::Delimeter(Delimeter::Plus)),
            '=' => Ok(Token::Delimeter(
                if self.iter.bubble_next_if(|ch| *ch == '=')?.is_some() {
                    Delimeter::EqualEqual
                } else {
                    Delimeter::Equal
                },
            )),
            '|' => Ok(Token::Delimeter(Delimeter::Pipe)),
            ';' => Ok(Token::Delimeter(Delimeter::Semicolon)),
            ':' => Ok(Token::Delimeter(Delimeter::Colon)),
            '<' => Ok(Token::Delimeter(Delimeter::OpenAngleBracket)),
            '>' => Ok(Token::Delimeter(Delimeter::CloseAngleBracket)),
            ',' => Ok(Token::Delimeter(Delimeter::Comma)),
            '.' => Ok(Token::Delimeter(Delimeter::Period)),
            '[' => Ok(Token::Delimeter(Delimeter::OpenBracket)),
            ']' => Ok(Token::Delimeter(Delimeter::CloseBracket)),
            '{' => Ok(Token::Delimeter(Delimeter::OpenBrace)),
            '}' => Ok(Token::Delimeter(Delimeter::CloseBrace)),
            '!' => Ok(Token::Delimeter(Delimeter::ExclamationMark)),
            '^' => Ok(Token::Delimeter(Delimeter::Carret)),
            '&' => Ok(Token::Delimeter(Delimeter::Ampersand)),
            '%' => Ok(Token::Delimeter(Delimeter::Percent)),
            '(' => Ok(Token::Delimeter(Delimeter::OpenParenthesis)),
            ')' => Ok(Token::Delimeter(Delimeter::CloseParenthesis)),
            '/' => Ok(Token::Delimeter(Delimeter::ForwardSlash)),
            ch if ch.is_ascii_whitespace() => unreachable!(),
            _ => unimplemented!(),
        }
    }
    fn lex_number(&mut self, ch: char) -> Result<([u8; 8], bool), LexerError<E>> {
        let mut string = String::new();
        let mut radix = 10;
        match ch {
            '0' => match self.iter.bubble_next_if(|ch| matches!(ch, 'x' | 'b'))? {
                Some('x') => radix = 16,
                Some('b') => radix = 2,
                None => string.push('0'),
                Some(_) => unreachable!(),
            },
            _ => string.push(ch),
        }
        match radix {
            16 => {
                while let Some(ch) = self.iter.bubble_next_if(|ch| ch.is_ascii_hexdigit())? {
                    string.push(ch);
                }
            }
            _ => {
                while let Some(ch) = self.iter.bubble_next_if(|ch| ch.is_ascii_digit())? {
                    string.push(ch);
                }
            }
        }
        if string.is_empty() {
            return Err(LexerError::Generic {
                message: format!("invalid num: base prefixes must be followed by a number"),
            });
        }
        if self.iter.bubble_next_if(|ch| *ch == '.')?.is_some() {
            if radix != 10 {
                return Err(LexerError::Generic {
                    message: format!("invalid f64: floats must be base 10"),
                });
            }
            string.push('.');
            while let Some(ch) = self.iter.bubble_next_if(|ch| ch.is_ascii_digit())? {
                string.push(ch);
            }
            if string.ends_with('.') {
                return Err(LexerError::Generic {
                    message: format!("invalid f64: floats must have a number following the period"),
                });
            }
            return match string.parse::<f64>() {
                Ok(n) => Ok((n.to_ne_bytes(), true)),
                Err(_) => Err(LexerError::Generic {
                    message: format!("invalid f64: \"{string}\""),
                }),
            };
        } else {
            return match string.parse::<i64>() {
                Ok(n) => Ok((n.to_ne_bytes(), false)),
                Err(_) => Err(LexerError::Generic {
                    message: format!("invalid i64: \"{string}\""),
                }),
            };
        }
    }
    fn lex_lexeme(&mut self, ch: char) -> Result<Lexeme, LexerError<E>> {
        let mut string = String::from(ch);
        while let Some(ch) = self
            .iter
            .bubble_next_if(|ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'))?
        {
            string.push(ch);
        }
        Ok(string)
    }
    fn lex_literal_string(&mut self, mut ch: char) -> Result<String, LexerError<E>> {
        let mut delimeter_hashtags = 0;
        while ch == '#' {
            delimeter_hashtags += 1;
            ch = self.iter.next().ok_or(LexerError::Incomplete).flatten()?;
        }
        if ch != '"' {
            return Err(LexerError::Generic {
                message: format!("Expected '\"' got {ch:?}"),
            });
        }
        let mut string = String::new();
        'char_push: loop {
            match self.iter.next().ok_or(LexerError::Incomplete).flatten()? {
                '"' => {
                    for i in 0..delimeter_hashtags {
                        match self.iter.next().ok_or(LexerError::Incomplete).flatten()? {
                            '#' => (),
                            ch => {
                                string.push('"');
                                for _ in 0..i {
                                    string.push('#');
                                }
                                string.push(ch);
                                continue 'char_push;
                            }
                        }
                    }
                    return Ok(string);
                }
                ch => string.push(ch),
            }
        }
    }
}

#[derive(Debug)]
pub enum LexerError<E> {
    Source(E),
    Generic { message: String },
    Incomplete,
}

impl<I, E> Iterator for Lexer<I, E>
where
    I: ErrorBubbledNLookahead<2, char, LexerError<E>>,
{
    type Item = Result<Token, LexerError<E>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}
