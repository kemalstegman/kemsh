use std::{
    convert::Infallible,
    iter::FusedIterator,
    marker::PhantomData,
    ops::ControlFlow::{Break, Continue},
};

pub trait LexemeLexer {
    type Lexeme<'a>;
    type Error;
    fn lex<'a>(
        lexable: &mut &'a str,
        known: usize,
    ) -> Result<Self::Lexeme<'a>, LexerError<Self::Error>>;
}

pub struct StandardLexemeLexer;

impl LexemeLexer for StandardLexemeLexer {
    type Error = Infallible;
    type Lexeme<'a> = &'a str;
    fn lex<'a>(
        lexable: &mut &'a str,
        known: usize,
    ) -> Result<Self::Lexeme<'a>, LexerError<Self::Error>> {
        let index = lexable[known..]
            .char_indices()
            .take_while(|&(_, ch)| ch == '_' || ch.is_ascii_alphanumeric())
            .last()
            .map(|(index, ch)| index + ch.len_utf8())
            .unwrap_or(0);
        let lexeme = &lexable[..known + index];
        *lexable = &lexable[known + index..];
        Ok(lexeme)
    }
}

pub trait NumberLexer {
    type Number<'a>;
    type Error;
    fn lex<'a>(
        lexable: &mut &'a str,
        zero_first: bool,
    ) -> Result<Self::Number<'a>, LexerError<Self::Error>>;
}

pub struct StandardNumberLexer;

impl NumberLexer for StandardNumberLexer {
    type Number<'a> = Number<'a>;
    type Error = ();
    fn lex<'a>(
        lexable: &mut &'a str,
        zero_first: bool,
    ) -> Result<Self::Number<'a>, LexerError<Self::Error>> {
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
}

pub trait DelimiterLexer {
    type Delimiter<'a>;
    type Error;
    fn lex<'a>(lexable: &mut &'a str) -> Result<Self::Delimiter<'a>, LexerError<Self::Error>>;
}

pub struct StandardDelimiterLexer;

impl DelimiterLexer for StandardDelimiterLexer {
    type Delimiter<'a> = &'a str;
    type Error = Infallible;
    fn lex<'a>(lexable: &mut &'a str) -> Result<Self::Delimiter<'a>, LexerError<Self::Error>> {
        const MULTI_DELIMS: &[&str] = &["..", "::", "==", "!=", "<=", ">=", "=>", "->"];
        for &delim in MULTI_DELIMS {
            if lexable.starts_with(delim) {
                *lexable = &lexable[delim.len()..];
                return Ok(delim);
            }
        }
        let ch = lexable.chars().next().unwrap();
        let len = ch.len_utf8();
        let delim = &lexable[..len];
        *lexable = &lexable[len..];
        Ok(delim)
    }
}

pub trait RawStringLexer {
    type RawString<'a>;
    type Error;
    fn lex<'a>(
        lexable: &mut &'a str,
        prefix: usize,
        seen_quote: bool,
    ) -> Result<Self::RawString<'a>, LexerError<Self::Error>>;
}

pub struct StandardRawStringLexer;

impl RawStringLexer for StandardRawStringLexer {
    type RawString<'a> = &'a str;
    type Error = char;
    fn lex<'a>(
        lexable: &mut &'a str,
        mut prefix: usize,
        seen_quote: bool,
    ) -> Result<Self::RawString<'a>, LexerError<Self::Error>> {
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
}

pub trait ShellLexer {
    type Shell<'a>;
    type Error;
    fn lex<'a>(lexable: &mut &'a str) -> Result<Self::Shell<'a>, LexerError<Self::Error>>;
}

pub struct StandardShellLexer;

impl ShellLexer for StandardShellLexer {
    type Shell<'a> = &'a str;
    type Error = Infallible;
    fn lex<'a>(lexable: &mut &'a str) -> Result<Self::Shell<'a>, LexerError<Self::Error>> {
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

pub trait EscapedStringLexer {
    type EscapedString<'a>;
    type Error;
    fn lex<'a>(_lexable: &mut &'a str) -> Result<Self::EscapedString<'a>, LexerError<Self::Error>> {
        todo!()
    }
}

pub struct StandardEscapedStringLexer;

impl EscapedStringLexer for StandardEscapedStringLexer {
    type EscapedString<'a> = ();
    type Error = ();
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Number<'a> {
    Integer(&'a str),
    Signed(&'a str),
    Unsigned(&'a str),
    Binary(&'a str),
    Hex(&'a str),
    Float(&'a str),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token<L, N, D, RS, ES, SH> {
    Lexeme(L),
    Number(N),
    Delimiter(D),
    RawString(RS),
    EscapedString(ES),
    Shell(SH),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LexerError<F> {
    Incomplete,
    Fatal(F),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum FatalLexerError<L, N, D, RS, ES, SH> {
    Incomplete,
    FirstCharUnexpected(char),
    Lexeme(L),
    Number(N),
    Delimiter(D),
    RawString(RS),
    EscapedString(ES),
    Shell(SH),
}

pub struct Lexer<'a, L, N, D, RS, ES, SH>
where
    L: LexemeLexer,
    N: NumberLexer,
    D: DelimiterLexer,
    RS: RawStringLexer,
    ES: EscapedStringLexer,
    SH: ShellLexer,
{
    lexable: &'a str,
    is_last_tok_lexeme: bool,
    _marker: PhantomData<(L, N, D, RS, ES, SH)>,
}

impl<'a, L, N, D, RS, ES, SH> Lexer<'a, L, N, D, RS, ES, SH>
where
    L: LexemeLexer,
    N: NumberLexer,
    D: DelimiterLexer,
    RS: RawStringLexer,
    ES: EscapedStringLexer,
    SH: ShellLexer,
{
    pub fn new(lexable: &'a str) -> Self {
        Self {
            lexable,
            is_last_tok_lexeme: false,
            _marker: PhantomData,
        }
    }
    pub fn lex(
        &mut self,
    ) -> Option<
        Result<
            Token<
                L::Lexeme<'a>,
                N::Number<'a>,
                D::Delimiter<'a>,
                RS::RawString<'a>,
                ES::EscapedString<'a>,
                SH::Shell<'a>,
            >,
            LexerError<
                FatalLexerError<L::Error, N::Error, D::Error, RS::Error, ES::Error, SH::Error>,
            >,
        >,
    > {
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
    fn lex_nonempty(
        &mut self,
    ) -> Result<
        Token<
            L::Lexeme<'a>,
            N::Number<'a>,
            D::Delimiter<'a>,
            RS::RawString<'a>,
            ES::EscapedString<'a>,
            SH::Shell<'a>,
        >,
        LexerError<FatalLexerError<L::Error, N::Error, D::Error, RS::Error, ES::Error, SH::Error>>,
    > {
        let mut char_iter = self.lexable.chars();
        let is_last_tok_lexeme = self.is_last_tok_lexeme;
        self.is_last_tok_lexeme = false;
        match char_iter.next().unwrap() {
            'e' => match char_iter.next() {
                Some('#') => match ES::lex(&mut self.lexable) {
                    Ok(es) => Ok(Token::EscapedString(es)),
                    Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                    Err(LexerError::Fatal(e)) => {
                        Err(LexerError::Fatal(FatalLexerError::EscapedString(e)))
                    }
                },
                Some('"') => match ES::lex(&mut self.lexable) {
                    Ok(es) => Ok(Token::EscapedString(es)),
                    Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                    Err(LexerError::Fatal(e)) => {
                        Err(LexerError::Fatal(FatalLexerError::EscapedString(e)))
                    }
                },
                None | Some(_) => match L::lex(&mut self.lexable, 'e'.len_utf8()) {
                    Ok(l) => {
                        self.is_last_tok_lexeme = true;
                        Ok(Token::Lexeme(l))
                    }
                    Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                    Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::Lexeme(e))),
                },
            },
            ch @ '_' | ch if ch.is_ascii_alphabetic() => match L::lex(&mut self.lexable, 1) {
                Ok(l) => {
                    self.is_last_tok_lexeme = true;
                    Ok(Token::Lexeme(l))
                }
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::Lexeme(e))),
            },
            '0' => match N::lex(&mut self.lexable, true) {
                Ok(n) => Ok(Token::Number(n)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::Number(e))),
            },
            ch if ch.is_ascii_digit() => match N::lex(&mut self.lexable, false) {
                Ok(n) => Ok(Token::Number(n)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::Number(e))),
            },
            ':' if !is_last_tok_lexeme => match SH::lex(&mut self.lexable) {
                Ok(sh) => Ok(Token::Shell(sh)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::Shell(e))),
            },
            '#' => match RS::lex(&mut self.lexable, 1, false) {
                Ok(rs) => Ok(Token::RawString(rs)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::RawString(e))),
            },
            '"' => match RS::lex(&mut self.lexable, 0, true) {
                Ok(rs) => Ok(Token::RawString(rs)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::RawString(e))),
            },
            ch if ch.is_ascii_punctuation() => match D::lex(&mut self.lexable) {
                Ok(d) => Ok(Token::Delimiter(d)),
                Err(LexerError::Incomplete) => Err(LexerError::Incomplete),
                Err(LexerError::Fatal(e)) => Err(LexerError::Fatal(FatalLexerError::Delimiter(e))),
            },
            ch => Err(LexerError::Fatal(FatalLexerError::FirstCharUnexpected(ch))),
        }
    }
}

impl<'a, L, N, D, RS, ES, SH> Iterator for Lexer<'a, L, N, D, RS, ES, SH>
where
    L: LexemeLexer,
    N: NumberLexer,
    D: DelimiterLexer,
    RS: RawStringLexer,
    ES: EscapedStringLexer,
    SH: ShellLexer,
{
    type Item = Result<
        Token<
            L::Lexeme<'a>,
            N::Number<'a>,
            D::Delimiter<'a>,
            RS::RawString<'a>,
            ES::EscapedString<'a>,
            SH::Shell<'a>,
        >,
        LexerError<FatalLexerError<L::Error, N::Error, D::Error, RS::Error, ES::Error, SH::Error>>,
    >;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

impl<'a, L, N, D, RS, ES, SH> FusedIterator for Lexer<'a, L, N, D, RS, ES, SH>
where
    L: LexemeLexer,
    N: NumberLexer,
    D: DelimiterLexer,
    RS: RawStringLexer,
    ES: EscapedStringLexer,
    SH: ShellLexer,
{
}
