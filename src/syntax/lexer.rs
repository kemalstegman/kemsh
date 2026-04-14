pub mod token;

use std::marker::PhantomData;

use crate::{abstract_lookahead::ErrorBubbledNLookahead, syntax::lexer::token::Token};

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
            '*' => Ok(Token::DELIM_ASTERISK),
            '-' => Ok(Token::DELIM_MINUS),
            '+' => Ok(Token::DELIM_PLUS),
            '=' => {
                if self.iter.bubble_next_if(|ch| *ch == '=')?.is_some() {
                    Ok(Token::DELIM_DOUBLEEQUAL)
                } else {
                    Ok(Token::DELIM_EQUAL)
                }
            }
            '|' => {
                if self.iter.bubble_next_if(|ch| *ch == '|')?.is_some() {
                    Ok(Token::DELIM_DOUBLEPIPE)
                } else {
                    Ok(Token::DELIM_PIPE)
                }
            }
            ';' => Ok(Token::DELIM_SEMICOLON),
            ':' => Ok(Token::DELIM_COLON),
            '<' => Ok(Token::DELIM_OPENANGLEBRACKET),
            '>' => {
                if self.iter.bubble_next_if(|ch| *ch == '>')?.is_some() {
                    Ok(Token::DELIM_DOUBLECLOSEANGLEBRACKET)
                } else {
                    Ok(Token::DELIM_CLOSEANGLEBRACKET)
                }
            }
            ',' => Ok(Token::DELIM_COMMA),
            '.' => Ok(Token::DELIM_PERIOD),
            '[' => Ok(Token::DELIM_OPENBRACKET),
            ']' => Ok(Token::DELIM_CLOSEBRACKET),
            '{' => Ok(Token::DELIM_OPENBRACE),
            '}' => {
                if self.iter.bubble_next_if(|ch| *ch == '#')?.is_some() {
                    Ok(Token::DELIM_CLOSEBRACEHASHTAG)
                } else {
                    Ok(Token::DELIM_CLOSEBRACE)
                }
            }
            '!' => Ok(Token::DELIM_EXCLAMATIONMARK),
            '&' => {
                if self.iter.bubble_next_if(|ch| *ch == '&')?.is_some() {
                    Ok(Token::DELIM_DOUBLEAMPERSAND)
                } else {
                    Ok(Token::DELIM_AMPERSAND)
                }
            }
            '%' => Ok(Token::DELIM_PERCENT),
            '(' => Ok(Token::DELIM_OPENPARENTHESIS),
            ')' => Ok(Token::DELIM_CLOSEPARENTHESIS),
            '/' => Ok(Token::DELIM_FORWARDSLASH),
            '"' => Ok(Token::RawString(
                RawStringLexer::new(&mut self.iter, 0)
                    .collect::<Result<String, LexerError<E>>>()?,
            )),
            '#' => {
                if self.iter.bubble_next_if(|ch| *ch == '{')?.is_some() {
                    Ok(Token::DELIM_HASHTAGOPENBRACE)
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
                Ok(Token::lexeme_from_string(string))
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
