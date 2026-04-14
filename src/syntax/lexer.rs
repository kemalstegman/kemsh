pub mod token;

use std::marker::PhantomData;

use crate::{
    abstract_lookahead::ErrorBubbledNLookahead,
    syntax::lexer::token::{Delimiter, Token},
};

/// todo!(): escaped strings, source errors
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
            '*' => Ok(Token::Delimiter(Delimiter::Asterisk)),
            '-' => Ok(Token::Delimiter(Delimiter::Minus)),
            '+' => Ok(Token::Delimiter(Delimiter::Plus)),
            '=' => {
                if self.iter.bubble_next_if(|ch| *ch == '=')?.is_some() {
                    Ok(Token::Delimiter(Delimiter::DoubleEqual))
                } else {
                    Ok(Token::Delimiter(Delimiter::Equal))
                }
            }
            '|' => {
                if self.iter.bubble_next_if(|ch| *ch == '|')?.is_some() {
                    Ok(Token::Delimiter(Delimiter::DoublePipe))
                } else {
                    Ok(Token::Delimiter(Delimiter::Pipe))
                }
            }
            ';' => Ok(Token::Delimiter(Delimiter::Semicolon)),
            ':' => Ok(Token::Delimiter(Delimiter::Colon)),
            '<' => Ok(Token::Delimiter(Delimiter::OpenAngleBracket)),
            '>' => {
                if self.iter.bubble_next_if(|ch| *ch == '>')?.is_some() {
                    Ok(Token::Delimiter(Delimiter::DoubleCloseAngleBracket))
                } else {
                    Ok(Token::Delimiter(Delimiter::CloseAngleBracket))
                }
            }
            ',' => Ok(Token::Delimiter(Delimiter::Comma)),
            '.' => Ok(Token::Delimiter(Delimiter::Period)),
            '[' => Ok(Token::Delimiter(Delimiter::OpenBracket)),
            ']' => Ok(Token::Delimiter(Delimiter::CloseBracket)),
            '{' => Ok(Token::Delimiter(Delimiter::OpenBrace)),
            '}' => {
                if self.iter.bubble_next_if(|ch| *ch == '#')?.is_some() {
                    Ok(Token::Delimiter(Delimiter::CloseBraceHashtag))
                } else {
                    Ok(Token::Delimiter(Delimiter::CloseBrace))
                }
            }
            '!' => Ok(Token::Delimiter(Delimiter::ExclamationMark)),
            '&' => {
                if self.iter.bubble_next_if(|ch| *ch == '&')?.is_some() {
                    Ok(Token::Delimiter(Delimiter::DoubleAmpersand))
                } else {
                    Ok(Token::Delimiter(Delimiter::Ampersand))
                }
            }
            '%' => Ok(Token::Delimiter(Delimiter::Percent)),
            '(' => Ok(Token::Delimiter(Delimiter::OpenParenthesis)),
            ')' => Ok(Token::Delimiter(Delimiter::CloseParenthesis)),
            '/' => Ok(Token::Delimiter(Delimiter::ForwardSlash)),
            '"' => Ok(Token::RawString(
                RawStringLexer::new(&mut self.iter, 0)
                    .collect::<Result<String, LexerError<E>>>()?,
            )),
            '#' => {
                if self.iter.bubble_next_if(|ch| *ch == '{')?.is_some() {
                    Ok(Token::Delimiter(Delimiter::HashtagOpenBrace))
                } else {
                    let mut prefix_hashtags = 1;
                    loop {
                        match self.iter.next().ok_or(LexerError::Incomplete).flatten()? {
                            '"' => break,
                            '#' => prefix_hashtags += 1,
                            ch => {
                                return Err(LexerError::Generic {
                                    message: format!("Expected '\"' or '#' got {ch:?}"),
                                });
                            }
                        }
                    }
                    Ok(Token::RawString(
                        RawStringLexer::new(&mut self.iter, prefix_hashtags).collect::<Result<
                            String,
                            LexerError<E>,
                        >>(
                        )?,
                    ))
                }
            }
            '0' if self.iter.bubble_next_if(|ch| *ch == 'x')?.is_some() => {
                let mut string = String::new();
                while let Some(ch) = self
                    .iter
                    .bubble_next_if(|ch| ch.is_ascii_hexdigit() || *ch == '_')?
                {
                    if ch == '_' {
                        continue;
                    }
                    string.push(ch);
                }
                if string.is_empty() {
                    return Err(LexerError::Generic {
                        message: format!("invalid integer: 0x prefix had no hexdigits following"),
                    });
                }
                // todo!() update this guard
                if self.iter.bubble_next_if(|ch| *ch == '.')?.is_some() {
                    return Err(LexerError::Generic {
                        message: format!("invalid float: floats must be base 10"),
                    });
                }
                match i64::from_str_radix(&string, 16) {
                    Ok(n) => Ok(Token::Integer(n)),
                    Err(_) => Err(LexerError::Generic {
                        message: format!("invalid integer: \"0x{string}\""),
                    }),
                }
            }
            '0' if self.iter.bubble_next_if(|ch| *ch == 'b')?.is_some() => {
                let mut string = String::new();
                while let Some(ch) = self
                    .iter
                    .bubble_next_if(|ch| *ch == '0' || *ch == '1' || *ch == '_')?
                {
                    if ch == '_' {
                        continue;
                    }
                    string.push(ch);
                }
                if string.is_empty() {
                    return Err(LexerError::Generic {
                        message: format!("invalid integer: 0b prefix had no binary following"),
                    });
                }
                // todo!() update this guard
                if self.iter.bubble_next_if(|ch| *ch == '.')?.is_some() {
                    return Err(LexerError::Generic {
                        message: format!("invalid float: floats must be base 10"),
                    });
                }
                match i64::from_str_radix(&string, 2) {
                    Ok(n) => Ok(Token::Integer(n)),
                    Err(_) => Err(LexerError::Generic {
                        message: format!("invalid integer: \"0b{string}\""),
                    }),
                }
            }
            ch if ch.is_ascii_digit() => {
                let mut string = String::from(ch);
                while let Some(ch) = self
                    .iter
                    .bubble_next_if(|ch| ch.is_ascii_digit() || *ch == '_')?
                {
                    if ch == '_' {
                        continue;
                    }
                    string.push(ch);
                }
                if self.iter.bubble_next_if(|ch| *ch == '.')?.is_some() {
                    string.push('.');
                    while let Some(ch) = self
                        .iter
                        .bubble_next_if(|ch| ch.is_ascii_digit() || *ch == '_')?
                    {
                        if ch == '_' {
                            continue;
                        }
                        string.push(ch);
                    }
                    if string.ends_with('.') {
                        return Err(LexerError::Generic {
                            message: format!("invalid float: period had no digits following"),
                        });
                    }
                    match string.parse::<f64>() {
                        Ok(n) => Ok(Token::Float(n)),
                        Err(_) => Err(LexerError::Generic {
                            message: format!("invalid float: \"{string}\""),
                        }),
                    }
                } else {
                    match i64::from_str_radix(&string, 10) {
                        Ok(n) => Ok(Token::Integer(n)),
                        Err(_) => Err(LexerError::Generic {
                            message: format!("invalid integer: \"{string}\""),
                        }),
                    }
                }
            }
            ch if ch.is_ascii_alphanumeric() || ch == '_' => {
                let mut string = String::from(ch);
                while let Some(ch) = self
                    .iter
                    .bubble_next_if(|ch| ch.is_ascii_alphanumeric() || *ch == '_')?
                {
                    string.push(ch);
                }
                Ok(Token::from_lexeme(string))
            }
            ch if ch.is_ascii_whitespace() => unreachable!(),
            _ => unimplemented!(),
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

/// LOOKAHEAD must be 1 or more
struct RawStringLexer<const LOOKAHEAD: usize, I, E>
where
    I: ErrorBubbledNLookahead<LOOKAHEAD, char, LexerError<E>>,
{
    prefix_hashtags: usize,
    suffix_hashtags: usize,
    iter: I,
    _marker: PhantomData<E>,
}

impl<const LOOKAHEAD: usize, I, E> RawStringLexer<LOOKAHEAD, I, E>
where
    I: ErrorBubbledNLookahead<LOOKAHEAD, char, LexerError<E>>,
{
    fn new(iter: I, prefix_hashtags: usize) -> Self {
        Self {
            prefix_hashtags,
            suffix_hashtags: 0,
            iter,
            _marker: PhantomData,
        }
    }
    fn next_char(&mut self) -> Result<Option<char>, LexerError<E>> {
        if self.suffix_hashtags > 0 {
            self.suffix_hashtags -= 1;
            return Ok(Some('#'));
        }
        match self.iter.next().ok_or(LexerError::Incomplete).flatten()? {
            '"' => {
                while self.prefix_hashtags > self.suffix_hashtags
                    && self.iter.bubble_next_if(|ch| *ch == '#')?.is_some()
                {
                    self.suffix_hashtags += 1;
                }
                if self.prefix_hashtags == self.suffix_hashtags {
                    return Ok(None);
                }
                Ok(Some('"'))
            }
            '\n' => Err(LexerError::Generic {
                message: String::from("raw strings cannot span multiple lines"),
            }),
            ch => Ok(Some(ch)),
        }
    }
}

impl<const LOOKAHEAD: usize, I, E> Iterator for RawStringLexer<LOOKAHEAD, I, E>
where
    I: ErrorBubbledNLookahead<LOOKAHEAD, char, LexerError<E>>,
{
    type Item = Result<char, LexerError<E>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_char().transpose()
    }
}
