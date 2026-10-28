use std::convert::Infallible;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token<'a, EscapedString, Lexeme, Delimiter, Shell> {
    RawString(&'a str),
    EscapedString(EscapedString),
    SInteger(i64),
    UInteger(u64),
    Float(f64),
    Lexeme(Lexeme),
    Delimiter(Delimiter),
    Shell(Shell),
}

pub trait ConsumingLexer {
    type Lexed<'a>;
    type Error;
    fn lex<'a>(lexable: &mut &'a str) -> Result<Self::Lexed<'a>, Self::Error>;
}

pub trait ParsingLexer {
    type Lexed<'a>;
    type Error;
    fn lex<'a>(lexable: &'a str) -> Result<Self::Lexed<'a>, Self::Error>;
}

pub struct EscapedStringLexer;

impl ParsingLexer for EscapedStringLexer {
    type Lexed<'a> = &'a str;
    type Error = Infallible;
    fn lex<'a>(lexable: &'a str) -> Result<Self::Lexed<'a>, Self::Error> {
        Ok(lexable)
    }
}

pub struct LexemeLexer;

impl ParsingLexer for LexemeLexer {
    type Lexed<'a> = &'a str;
    type Error = Infallible;
    fn lex<'a>(lexable: &'a str) -> Result<Self::Lexed<'a>, Self::Error> {
        Ok(lexable)
    }
}

pub struct DelimiterLexer;

impl ConsumingLexer for DelimiterLexer {
    type Lexed<'a> = &'a str;
    type Error = Infallible;
    fn lex<'a>(lexable: &mut &'a str) -> Result<Self::Lexed<'a>, Self::Error> {
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

pub struct ShellLexer;

impl ParsingLexer for ShellLexer {
    type Lexed<'a> = &'a str;
    type Error = Infallible;
    fn lex<'a>(lexable: &'a str) -> Result<Self::Lexed<'a>, Self::Error> {
        Ok(lexable)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LexerError<ESE, LE, DE, SHE> {
    Incomplete,
    UnexpectedFirstChar(char),
    ExpectedCharGotChar(&'static [char], char),
    Lexeme(LE),
    Delimiter(DE),
    EscapedString(ESE),
    Shell(SHE),
    SInteger(String),
    UInteger(String),
    Float(String),
    Binary(String),
    Hexadecimal(String),
}

pub struct Lexer<'a, ES, L, D, SH> {
    lexable: &'a str,
    last_tok_lexeme: bool,
    _marker: std::marker::PhantomData<(ES, L, D, SH)>,
}
impl<'a, ESL, LL, DL, SHL> Lexer<'a, ESL, LL, DL, SHL>
where
    ESL: ParsingLexer,
    LL: ParsingLexer,
    DL: ConsumingLexer,
    SHL: ParsingLexer,
{
    pub fn new(lexable: &'a str) -> Self {
        Self {
            lexable,
            last_tok_lexeme: false,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn lex(
        &mut self,
    ) -> Option<
        Result<
            Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
            LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
        >,
    > {
        if self.lexable.is_empty() {
            return None;
        }
        let res = self.lex_();
        if res.is_err() {
            self.lexable = &self.lexable[..0];
        }
        Some(res)
    }
    fn lex_signed_integer(
        &mut self,
        integer_bytes: usize,
        consume_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let integer_string = &self.lexable[..integer_bytes];
        match integer_string.parse::<i64>() {
            Ok(n) => {
                self.lexable = &self.lexable[consume_bytes..];
                Ok(Token::SInteger(n))
            }
            Err(e) => Err(LexerError::SInteger(format!("{e}: {integer_string}"))),
        }
    }
    fn lex_unsigned_integer(
        &mut self,
        integer_bytes: usize,
        consume_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let integer_string = &self.lexable[..integer_bytes];
        match integer_string.parse::<u64>() {
            Ok(n) => {
                self.lexable = &self.lexable[consume_bytes..];
                Ok(Token::UInteger(n))
            }
            Err(e) => Err(LexerError::UInteger(format!("{e}: {integer_string}"))),
        }
    }
    fn lex_float(
        &mut self,
        float_bytes: usize,
        consume_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let float_string = &self.lexable[..float_bytes];
        match float_string.parse::<f64>() {
            Ok(n) => {
                self.lexable = &self.lexable[consume_bytes..];
                Ok(Token::Float(n))
            }
            Err(e) => Err(LexerError::Float(format!("{e}: {float_string}"))),
        }
    }
    fn lex_binary(
        &mut self,
        binary_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let binary_string = &self.lexable[2..binary_bytes];
        match u64::from_str_radix(binary_string, 2) {
            Ok(n) => {
                self.lexable = &self.lexable[binary_bytes..];
                Ok(Token::UInteger(n))
            }
            Err(e) => Err(LexerError::Binary(format!("{e}: {binary_string}"))),
        }
    }
    fn lex_hexadecimal(
        &mut self,
        hexadecimal_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let hexadecimal_string = &self.lexable[2..hexadecimal_bytes];
        match u64::from_str_radix(hexadecimal_string, 16) {
            Ok(n) => {
                self.lexable = &self.lexable[hexadecimal_bytes..];
                Ok(Token::UInteger(n))
            }
            Err(e) => Err(LexerError::Hexadecimal(format!(
                "{e}: {hexadecimal_string}"
            ))),
        }
    }
    fn lex_lexeme(
        &mut self,
        lexeme_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let lexeme_string = &self.lexable[..lexeme_bytes];
        match LL::lex(lexeme_string) {
            Ok(l) => {
                self.lexable = &self.lexable[lexeme_bytes..];
                self.last_tok_lexeme = true;
                return Ok(Token::Lexeme(l));
            }
            Err(e) => return Err(LexerError::Lexeme(e)),
        }
    }
    fn _lex_escaped_string(
        &mut self,
        _escaped_string_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        todo!()
    }
    fn lex_shell(
        &mut self,
        shell_bytes: usize,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        let shell_string = &self.lexable[1..shell_bytes];
        self.lexable = &self.lexable[shell_bytes..];
        match SHL::lex(shell_string) {
            Ok(sh) => {
                self.last_tok_lexeme = true;
                return Ok(Token::Shell(sh));
            }
            Err(e) => return Err(LexerError::Shell(e)),
        }
    }
    fn lex_(
        &mut self,
    ) -> Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    > {
        self.lexable = self.lexable.trim_ascii();
        enum State {
            Start,
            Lexeme,
            NumberZero,
            NumberDigits,
            HexDigits,
            BinaryDigits,
            FloatDot,
            FloatDigits,
            SInteger1,
            SInteger2,
            UInteger1,
            UInteger2,
            RawStringPrefix { prefix: usize },
            RawString { prefix: usize },
            RawStringSuffix { prefix: usize, suffix: usize },
            E,
            Shell,
        }
        let mut state = State::Start;
        let last_tok_lexeme = self.last_tok_lexeme;
        self.last_tok_lexeme = false;
        for (index, ch) in self.lexable.char_indices() {
            state = match state {
                State::Start if ch == 'e' => State::E,
                State::Start if ch.is_ascii_alphabetic() || ch == '_' => State::Lexeme,
                State::Start if ch == '0' => State::NumberZero,
                State::Start if ch.is_ascii_digit() => State::NumberDigits,
                State::Start if ch == ':' && !last_tok_lexeme => State::Shell,
                State::Start if ch == '#' => State::RawStringPrefix { prefix: 1 },
                State::Start if ch == '"' => State::RawString { prefix: 0 },
                State::Start if ch.is_ascii_punctuation() => match DL::lex(&mut self.lexable) {
                    Err(e) => return Err(LexerError::Delimiter(e)),
                    Ok(l) => return Ok(Token::Delimiter(l)),
                },
                State::Start => {
                    return Err(LexerError::UnexpectedFirstChar(ch));
                }
                State::Lexeme => {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        continue;
                    } else {
                        return self.lex_lexeme(index);
                    }
                }
                State::NumberZero => match ch {
                    'x' | 'X' => State::HexDigits,
                    'b' | 'B' => State::BinaryDigits,
                    '.' => {
                        if !self.lexable[index + 1..].is_empty()
                            && self.lexable[index + 1..]
                                .chars()
                                .next()
                                .unwrap()
                                .is_ascii_digit()
                        {
                            State::FloatDot
                        } else {
                            self.lexable = &self.lexable[index..];
                            return Ok(Token::SInteger(0));
                        }
                    }
                    ch if ch.is_ascii_digit() => State::NumberDigits,
                    'i' => State::SInteger1,
                    'u' => State::UInteger1,
                    _ => {
                        self.lexable = &self.lexable[index..];
                        return Ok(Token::SInteger(0));
                    }
                },
                State::NumberDigits => match ch {
                    ch if ch.is_ascii_digit() => continue,
                    '.' => {
                        if !self.lexable[index + 1..].is_empty()
                            && self.lexable[index + 1..]
                                .chars()
                                .next()
                                .unwrap()
                                .is_ascii_digit()
                        {
                            State::FloatDot
                        } else {
                            return self.lex_signed_integer(index, index);
                        }
                    }
                    'i' => State::SInteger1,
                    'u' => State::UInteger1,
                    _ => return self.lex_signed_integer(index, index),
                },
                State::HexDigits => {
                    if ch.is_ascii_hexdigit() {
                        continue;
                    } else {
                        return self.lex_hexadecimal(index);
                    }
                }
                State::BinaryDigits => {
                    if ch == '0' || ch == '1' {
                        continue;
                    } else {
                        return self.lex_binary(index);
                    }
                }
                State::FloatDot => State::FloatDigits,
                State::FloatDigits => {
                    if ch.is_ascii_digit() {
                        continue;
                    } else {
                        return self.lex_float(index, index);
                    }
                }
                State::SInteger1 if ch == '6' => State::SInteger2,
                State::SInteger1 => {
                    return Err(LexerError::ExpectedCharGotChar(&['6'], ch));
                }
                State::SInteger2 if ch == '4' => {
                    return self.lex_signed_integer(index - 2, index + 1);
                }
                State::SInteger2 => {
                    return Err(LexerError::ExpectedCharGotChar(&['4'], ch));
                }
                State::UInteger1 if ch == '6' => State::UInteger2,
                State::UInteger1 => {
                    return Err(LexerError::ExpectedCharGotChar(&['6'], ch));
                }
                State::UInteger2 if ch == '4' => {
                    return self.lex_unsigned_integer(index - 2, index + 1);
                }
                State::UInteger2 => {
                    return Err(LexerError::ExpectedCharGotChar(&['4'], ch));
                }
                State::E if ch.is_ascii_alphanumeric() || ch == '_' => State::Lexeme,
                State::E if ch == '#' => todo!(),
                State::E if ch == '"' => todo!(),
                State::E => {
                    let lexeme_string = &self.lexable[..index];
                    self.lexable = &self.lexable[index..];
                    match LL::lex(lexeme_string) {
                        Ok(l) => {
                            self.last_tok_lexeme = true;
                            return Ok(Token::Lexeme(l));
                        }
                        Err(e) => return Err(LexerError::Lexeme(e)),
                    }
                }
                State::RawStringPrefix { prefix } if ch == '#' => {
                    State::RawStringPrefix { prefix: prefix + 1 }
                }
                State::RawStringPrefix { prefix } if ch == '"' => State::RawString { prefix },
                State::RawStringPrefix { prefix: _ } => {
                    return Err(LexerError::ExpectedCharGotChar(&['#', '\"'], ch));
                }
                State::RawString { prefix } if ch == '"' => {
                    if prefix == 0 {
                        let token_end = index + '"'.len_utf8();
                        let delim_bytes = '"'.len_utf8();
                        let raw_string_string = &self.lexable[delim_bytes..token_end - delim_bytes];
                        self.lexable = &self.lexable[token_end..];
                        return Ok(Token::RawString(raw_string_string));
                    } else {
                        State::RawStringSuffix { prefix, suffix: 0 }
                    }
                }
                State::RawString { prefix: _ } => continue,
                State::RawStringSuffix { prefix, suffix: _ } if ch == '"' => {
                    State::RawStringSuffix { prefix, suffix: 0 }
                }
                State::RawStringSuffix { prefix, suffix } if ch == '#' => {
                    let suffix = suffix + 1;
                    if prefix == suffix {
                        let token_end = index + '#'.len_utf8();
                        let delim_bytes = '"'.len_utf8() + '#'.len_utf8() * prefix;
                        let raw_string_string = &self.lexable[delim_bytes..token_end - delim_bytes];
                        self.lexable = &self.lexable[token_end..];
                        return Ok(Token::RawString(raw_string_string));
                    } else {
                        State::RawStringSuffix { prefix, suffix }
                    }
                }
                State::RawStringSuffix { prefix, suffix: _ } => State::RawString { prefix },
                State::Shell => {
                    if ch != ';' {
                        continue;
                    } else {
                        return self.lex_shell(index);
                    }
                }
            };
        }
        match state {
            State::Start => unreachable!(),
            State::E | State::Lexeme => {
                return self.lex_lexeme(self.lexable.len());
            }
            State::NumberZero => {
                self.lexable = &self.lexable[..0];
                return Ok(Token::SInteger(0));
            }
            State::NumberDigits => {
                return self.lex_signed_integer(self.lexable.len(), self.lexable.len());
            }
            State::HexDigits => {
                return self.lex_hexadecimal(self.lexable.len());
            }
            State::BinaryDigits => {
                return self.lex_binary(self.lexable.len());
            }
            State::FloatDot => unreachable!(),
            State::FloatDigits => {
                return self.lex_float(self.lexable.len(), self.lexable.len());
            }
            State::SInteger1
            | State::SInteger2
            | State::UInteger1
            | State::UInteger2
            | State::RawString { prefix: _ }
            | State::RawStringPrefix { prefix: _ }
            | State::RawStringSuffix {
                prefix: _,
                suffix: _,
            } => Err(LexerError::Incomplete),
            State::Shell => {
                return self.lex_shell(self.lexable.len());
            }
        }
    }
}

impl<'a, ESL, LL, DL, SHL> Iterator for Lexer<'a, ESL, LL, DL, SHL>
where
    ESL: ParsingLexer,
    LL: ParsingLexer,
    DL: ConsumingLexer,
    SHL: ParsingLexer,
{
    type Item = Result<
        Token<'a, ESL::Lexed<'a>, LL::Lexed<'a>, DL::Lexed<'a>, SHL::Lexed<'a>>,
        LexerError<ESL::Error, LL::Error, DL::Error, SHL::Error>,
    >;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

impl<'a, ESL, LL, DL, SHL> std::iter::FusedIterator for Lexer<'a, ESL, LL, DL, SHL>
where
    ESL: ParsingLexer,
    LL: ParsingLexer,
    DL: ConsumingLexer,
    SHL: ParsingLexer,
{
}
