use std::{
    convert::Infallible,
    iter::FusedIterator,
    ops::ControlFlow::{Break, Continue},
};

pub use delimiter::Delimiter;
pub use lexeme::Lexeme;

use crate::lexer::escaped_string::lex_escaped_string;

mod delimiter;
mod escaped_string;
mod lexeme;

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError<F> {
    Incomplete,
    Fatal(F),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Number<'a> {
    Integer(&'a str),
    Signed(&'a str),
    Unsigned(&'a str),
    Binary(&'a str),
    Hex(&'a str),
    Float(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Lexeme(Lexeme<'a>),
    Number(Number<'a>),
    Delimiter(Delimiter),
    RawString(&'a str),
    EscapedString(Infallible),
    Shell(&'a str),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum FatalLexerError {
    Incomplete,
    FirstCharUnexpected(char),
    Delimiter(char),
    RawString(char),
    EscapedString(Infallible),
}

pub struct Lexer<'a> {
    lexable: &'a str,
    is_last_tok_lexeme: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(lexable: &'a str) -> Self {
        Self {
            lexable,
            is_last_tok_lexeme: false,
        }
    }
    pub fn lex(&mut self) -> Option<Result<Token<'a>, LexerError<FatalLexerError>>> {
        self.lexable = self.lexable.trim_ascii_start();
        if self.lexable.is_empty() {
            return None;
        }
        let ret = self.lex_nonempty();
        if ret.is_err() {
            self.lexable = &self.lexable[..0];
        }
        Some(ret)
    }
    fn lex_nonempty(&mut self) -> Result<Token<'a>, LexerError<FatalLexerError>> {
        let mut char_iter = self.lexable.chars();
        let is_last_tok_lexeme = self.is_last_tok_lexeme;
        self.is_last_tok_lexeme = false;
        match char_iter.next().unwrap() {
            'e' => match char_iter.next() {
                Some('#') => lex_escaped_string(&mut self.lexable),
                Some('"') => lex_escaped_string(&mut self.lexable),
                None | Some(_) => {
                    let l = Lexeme::lex_assume_first_byte(&mut self.lexable);
                    self.is_last_tok_lexeme = true;
                    Ok(Token::Lexeme(l))
                }
            },
            ch @ '_' | ch if ch.is_ascii_alphabetic() => {
                let l = Lexeme::lex_assume_first_byte(&mut self.lexable);
                self.is_last_tok_lexeme = true;
                Ok(Token::Lexeme(l))
            }
            '0' => match Self::lex_number(&mut self.lexable, true) {
                Ok(n) => Ok(Token::Number(n)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
            },
            ch if ch.is_ascii_digit() => match Self::lex_number(&mut self.lexable, false) {
                Ok(n) => Ok(Token::Number(n)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
            },
            ':' if !is_last_tok_lexeme => match Self::lex_shell(&mut self.lexable) {
                Ok(sh) => Ok(Token::Shell(sh)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
            },
            '#' => match Self::lex_raw_string(&mut self.lexable, 1, false) {
                Ok(rs) => Ok(Token::RawString(rs)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::RawString(e))),
            },
            '"' => match Self::lex_raw_string(&mut self.lexable, 0, true) {
                Ok(rs) => Ok(Token::RawString(rs)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::RawString(e))),
            },
            '{' => match char_iter.next() {
                Some('[') => {
                    self.lexable = &self.lexable[2..];
                    Ok(Token::Delimiter(Delimiter::OpenBraceBracket))
                }
                Some(_) | None => {
                    self.lexable = &self.lexable[1..];
                    Ok(Token::Delimiter(Delimiter::OpenBrace))
                }
            },
            '}' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::CloseBrace))
            }
            '[' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::OpenSquareBracket))
            }
            ']' => match char_iter.next() {
                Some('}') => {
                    self.lexable = &self.lexable[2..];
                    Ok(Token::Delimiter(Delimiter::CloseBracketBrace))
                }
                Some(_) | None => {
                    self.lexable = &self.lexable[1..];
                    Ok(Token::Delimiter(Delimiter::CloseSquareBracket))
                }
            },
            '(' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::OpenParenthesis))
            }
            ')' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::CloseParenthesis))
            }
            '<' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::OpenAngleBracket))
            }
            '>' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::CloseAngleBracket))
            }
            ',' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Comma))
            }
            '.' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Period))
            }
            ';' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Semicolon))
            }
            ':' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Colon))
            }
            '-' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::HyphenMinus))
            }
            '+' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Plus))
            }
            '=' => match char_iter.next() {
                Some('=') => {
                    self.lexable = &self.lexable[2..];
                    Ok(Token::Delimiter(Delimiter::EqualEqual))
                }
                Some(_) | None => {
                    self.lexable = &self.lexable[1..];
                    Ok(Token::Delimiter(Delimiter::Equal))
                }
            },
            '/' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::ForwardSlash))
            }
            '\\' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::BackSlash))
            }
            '*' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Asterisk))
            }
            '!' => {
                self.lexable = &self.lexable[1..];
                Ok(Token::Delimiter(Delimiter::Bang))
            }
            ch => Err(LexerError::Fatal(FatalLexerError::FirstCharUnexpected(ch))),
        }
    }
    fn lex_number<'b>(
        lexable: &mut &'b str,
        zero_first: bool,
    ) -> Result<Number<'b>, LexerError<Infallible>> {
        if zero_first {
            match lexable['0'.len_utf8()..].chars().next() {
                None => {
                    *lexable = &lexable[1..];
                    return Ok(Number::Integer("0"));
                }
                Some('x') | Some('X') => {
                    let index = 2 + lexable[2..]
                        .char_indices()
                        .take_while(|(_, ch)| ch.is_ascii_hexdigit())
                        .last()
                        .map(|(index, ch)| index + ch.len_utf8())
                        .unwrap_or(0);
                    let hex_string = &lexable[2..index];
                    *lexable = &lexable[index..];
                    return Ok(Number::Hex(hex_string));
                }
                Some('b') | Some('B') => {
                    let index = 2 + lexable[2..]
                        .char_indices()
                        .take_while(|&(_, ch)| ch == '0' || ch == '1')
                        .last()
                        .map(|(index, ch)| index + ch.len_utf8())
                        .unwrap_or(0);
                    let binary_string = &lexable[2..index];
                    *lexable = &lexable[index..];
                    return Ok(Number::Binary(binary_string));
                }
                Some(_) => (),
            }
        }
        let mut is_float = false;
        let mut number_string_index = lexable.len();
        for (index, ch) in lexable.char_indices() {
            match ch {
                ch if ch.is_ascii_digit() => continue,
                '.' if !is_float
                    && lexable[index + ('.'.len_utf8())..]
                        .chars()
                        .next()
                        .map(|ch| ch.is_ascii_digit())
                        .unwrap_or(false) =>
                {
                    is_float = true
                }
                ch @ ('i' | 'u') if lexable[index + 1..].starts_with("64") => {
                    let number_string = &lexable[..index];
                    *lexable = &lexable[index + 3..];
                    match ch {
                        'i' => return Ok(Number::Signed(number_string)),
                        'u' => return Ok(Number::Unsigned(number_string)),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    number_string_index = index;
                    break;
                }
            }
        }
        let number_string = &lexable[..number_string_index];
        *lexable = &lexable[number_string_index..];
        if is_float {
            Ok(Number::Float(number_string))
        } else {
            Ok(Number::Integer(number_string))
        }
    }
    fn lex_raw_string<'b>(
        lexable: &mut &'b str,
        mut prefix: usize,
        seen_quote: bool,
    ) -> Result<&'b str, LexerError<char>> {
        if !seen_quote {
            for ch in lexable[prefix * ('#'.len_utf8())..].chars() {
                match ch {
                    '#' => prefix += 1,
                    '"' => break,
                    ch => return Err(LexerError::Fatal(ch)),
                }
            }
        }
        let index = match lexable[prefix * ('#'.len_utf8()) + ('"'.len_utf8())..]
            .char_indices()
            .try_fold(
                (0, false),
                |(mut suffix_seen, seen_quote), (index, ch)| match ch {
                    '"' => {
                        if suffix_seen == prefix {
                            Break(index + '"'.len_utf8())
                        } else {
                            Continue((0, true))
                        }
                    }
                    '#' if seen_quote => {
                        suffix_seen += 1;
                        if suffix_seen == prefix {
                            Break(index + '#'.len_utf8())
                        } else {
                            Continue((suffix_seen, true))
                        }
                    }
                    _ => Continue((0, false)),
                },
            ) {
            Continue(_) => return Err(LexerError::Incomplete),
            Break(index) => index + prefix * ('#'.len_utf8()) + ('"'.len_utf8()),
        };
        // prefix == suffix
        let raw_string = &lexable
            [prefix + ('"'.len_utf8())..index - prefix * ('#'.len_utf8()) - ('"'.len_utf8())];
        *lexable = &lexable[index..];
        Ok(raw_string)
    }
    fn lex_shell<'b>(lexable: &mut &'b str) -> Result<&'b str, LexerError<Infallible>> {
        match lexable[':'.len_utf8()..]
            .char_indices()
            .find(|&(_, ch)| ch == ';')
        {
            None => {
                let shell = &lexable[':'.len_utf8()..lexable.len()];
                *lexable = &lexable[..0];
                Ok(shell)
            }
            Some((index, _)) => {
                let index = index + ';'.len_utf8();
                let shell = &lexable[':'.len_utf8()..index];
                *lexable = &lexable[index..];
                Ok(shell)
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError<FatalLexerError>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

impl<'a> FusedIterator for Lexer<'a> {}
