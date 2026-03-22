//! Provides the notable types Lexer and LexError.

use std::{hint::unreachable_unchecked, iter::Peekable};

use crate::analyzer::AnalyzerError;

pub mod token;
use token::{Token, TokenDelimeter};

#[derive(Debug)]
pub struct LexError {
    message: String,
}

/// An iterator adaptor that consumes characters to produce
/// `Token`s. The iterator used to construct the `Lexer` will
/// consume all elements if no errors are produced by the `Lexer`
/// or the source iterator. If the `Lexer` returns an error, any
/// subsequent iterations have undefined behavior. Thus, with
/// correct usage, iteration will stop when the `Lexer` no
/// longer produces any `Token`s or when the `Lexer` produces
/// an error.
pub struct Lexer<I, E>
where
    I: IntoIterator<Item = Result<char, AnalyzerError<E>>>,
{
    iter: Peekable<I::IntoIter>,
}

impl<I, E: std::fmt::Debug> Lexer<I, E>
where
    I: IntoIterator<Item = Result<char, AnalyzerError<E>>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.into_iter().peekable(),
        }
    }
    fn lex(&mut self) -> Option<Result<Token, AnalyzerError<E>>> {
        loop {
            return match self.iter.next()? {
                Err(e) => Some(Err(e)),
                Ok(ch) if ch.is_whitespace() => continue,
                Ok(ch) => Some(self.lex_token(ch)),
            };
        }
    }
    /// Similar to next_char, flattens a None from the source iterator into a
    /// LexerError::Incomplete. On an error, calls next to return an owned error.
    fn peek_char(&mut self) -> Result<char, AnalyzerError<E>> {
        match self.iter.peek().ok_or(AnalyzerError::Incomplete)?.as_ref() {
            Ok(&ch) => Ok(ch),
            Err(_) => self.iter.next().unwrap(),
        }
    }
    /// Flattens a None from the source iterator into a LexerError::Incomplete
    fn next_char(&mut self) -> Result<char, AnalyzerError<E>> {
        self.iter.next().ok_or(AnalyzerError::Incomplete).flatten()
    }
    /// Like next_char, but returns None if func returns false. Bubbles up errors.
    fn next_char_if(
        &mut self,
        func: impl FnOnce(char) -> bool,
    ) -> Result<Option<char>, AnalyzerError<E>> {
        if func(self.peek_char()?) {
            self.iter.next().transpose()
        } else {
            Ok(None)
        }
    }
    fn next_char_if_eq(&mut self, ch: char) -> Result<Option<char>, AnalyzerError<E>> {
        self.next_char_if(|peek_ch| peek_ch == ch)
    }
    fn lex_token(&mut self, first_char: char) -> Result<Token, AnalyzerError<E>> {
        match first_char {
            ' ' | '\t' | '\n' | '\r' => unreachable!(),
            '"' => self
                .lex_string(0, false)
                .map(|string| Token::String(string)),
            '#' => {
                let delimeter_hashtags = self.consume_string_start_delimeter()? + 1;
                self.lex_string(delimeter_hashtags, false)
                    .map(|string| Token::String(string))
            }
            'e' if self.next_char_if_eq('"')?.is_some() => {
                self.lex_string(0, true).map(|string| Token::String(string))
            }
            'e' if self.next_char_if_eq('#')?.is_some() => {
                let delimeter_hashtags = self.consume_string_start_delimeter()? + 1;
                self.lex_string(delimeter_hashtags, false)
                    .map(|string| Token::String(string))
            }
            ch @ '0'..='9' => {
                let number_string = String::from(ch);
                self.lex_number(number_string)
            }
            ch @ ('a'..='z' | 'A'..='Z' | '_') => {
                let mut lexeme = String::from(ch);
                self.lex_lexeme(&mut lexeme)?;
                Ok(Token::from_lexeme(lexeme))
            }
            '!' => Ok(Token::Delimeter(TokenDelimeter::ExclamationMark)),
            '^' => Ok(Token::Delimeter(TokenDelimeter::Carret)),
            '&' => match self.next_char_if_eq('&')? {
                Some(_) => Ok(Token::Delimeter(TokenDelimeter::AmpersandAmpersand)),
                None => Ok(Token::Delimeter(TokenDelimeter::Ampersand)),
            },
            '*' => Ok(Token::Delimeter(TokenDelimeter::Asterisk)),
            '(' => Ok(Token::Delimeter(TokenDelimeter::OpenParenthesis)),
            ')' => Ok(Token::Delimeter(TokenDelimeter::CloseParenthesis)),
            '-' => Ok(Token::Delimeter(TokenDelimeter::Minus)),
            '+' => Ok(Token::Delimeter(TokenDelimeter::Plus)),
            '=' => match self.next_char_if_eq('=')? {
                Some(_) => Ok(Token::Delimeter(TokenDelimeter::EqualEqual)),
                None => Ok(Token::Delimeter(TokenDelimeter::Equal)),
            },
            '|' => match self.next_char_if_eq('|')? {
                Some(_) => Ok(Token::Delimeter(TokenDelimeter::PipePipe)),
                None => Ok(Token::Delimeter(TokenDelimeter::Pipe)),
            },
            '[' => Ok(Token::Delimeter(TokenDelimeter::OpenBracket)),
            ']' => Ok(Token::Delimeter(TokenDelimeter::CloseBracket)),
            '{' => Ok(Token::Delimeter(TokenDelimeter::OpenBrace)),
            '}' => Ok(Token::Delimeter(TokenDelimeter::CloseBrace)),
            ';' => Ok(Token::Delimeter(TokenDelimeter::Semicolon)),
            ':' => Ok(Token::Delimeter(TokenDelimeter::Colon)),
            '<' => Ok(Token::Delimeter(TokenDelimeter::OpenAngleBracket)),
            '>' => Ok(Token::Delimeter(TokenDelimeter::CloseAngleBracket)),
            ',' => Ok(Token::Delimeter(TokenDelimeter::Comma)),
            '.' => Ok(Token::Delimeter(TokenDelimeter::Period)),
            '/' => Ok(Token::Delimeter(TokenDelimeter::ForwardSlash)),
            ch => Err(AnalyzerError::Lexer(LexError {
                message: format!("Unexpected character: {:?}", ch),
            })),
        }
    }
    /// consumes consecutive `#` and then one `"`. success if the `"` was consumed
    /// and returns the number of `#` consumed.
    fn consume_string_start_delimeter(&mut self) -> Result<usize, AnalyzerError<E>> {
        let mut hashtag_count = 0;
        loop {
            match self.next_char()? {
                '#' => hashtag_count += 1,
                '"' => return Ok(hashtag_count),
                ch => {
                    return Err(AnalyzerError::Lexer(LexError {
                        message: format!("Expected '#' or '\"', got {ch:?}"),
                    }));
                }
            }
        }
    }

    /// consumes characters into a String until `delimeter_hashtags` `#`
    /// and one `"` is consumed. the `#` and `"` from the delimeter will
    /// not be included in the string, but other `#` and `"` will be
    /// included.
    fn lex_string(
        &mut self,
        delimeter_hashtags: usize,
        _escaped: bool, // todo!()
    ) -> Result<String, AnalyzerError<E>> {
        let mut string = String::new();
        'charpush: loop {
            match self.next_char()? {
                // '\\' if _escaped => todo!(),
                '"' => {
                    for i in 0..delimeter_hashtags {
                        match self.next_char()? {
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
                    return Ok(string);
                }
                ch => string.push(ch),
            }
        }
    }

    fn lex_lexeme(&mut self, string: &mut String) -> Result<(), AnalyzerError<E>> {
        while let Some(ch) = self.next_char_if(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
            _ => false,
        })? {
            string.push(ch);
        }
        Ok(())
    }

    fn lex_number(&mut self, mut number_string: String) -> Result<Token, AnalyzerError<E>> {
        let mut is_float = false;
        while let Some(ch) = self.next_char_if(|ch| match ch {
            '0'..='9' => true,
            _ => false,
        })? {
            number_string.push(ch);
        }
        if let Some('.') = self.next_char_if_eq('.')? {
            is_float = true;
            while let Some(ch) = self.next_char_if(|ch| match ch {
                '0'..='9' => true,
                _ => false,
            })? {
                number_string.push(ch);
            }
        }
        if is_float {
            match number_string.parse::<f64>() {
                Ok(n) => Ok(Token::Float(n)),
                Err(_) => Err(AnalyzerError::Lexer(LexError {
                    message: format!("Invalid (f64) number: {:?}", number_string),
                })),
            }
        } else {
            match number_string.parse::<i64>() {
                Ok(n) => Ok(Token::Integer(n)),
                Err(_) => Err(AnalyzerError::Lexer(LexError {
                    message: format!("Invalid (i64) number: {:?}", number_string),
                })),
            }
        }
    }
}

impl<I, E: std::fmt::Debug> Iterator for Lexer<I, E>
where
    I: IntoIterator<Item = Result<char, AnalyzerError<E>>>,
{
    type Item = Result<Token, AnalyzerError<E>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}
