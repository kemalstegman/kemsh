#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token<'a, ES, L, D, SH> {
    RawString(&'a str),
    EscapedString(ES),
    SInteger(i64),
    UInteger(u64),
    Float(f64),
    Lexeme(L),
    Delimiter(D),
    Shell(SH),
}

// a better name is needed. this is a trait describing
// something that represents a Token (above) that can be
// created via some lexing function given a lexable string
pub trait Lex<'a>: Sized {
    type Err;
    fn lex(lexable: &'a str) -> Result<Self, Self::Err>;
}

impl<'a> Lex<'a> for &'a str {
    type Err = std::convert::Infallible;
    fn lex(lexable: &'a str) -> Result<Self, Self::Err> {
        Ok(lexable)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LexerError<'a, ES, L, D, SH>
where
    ES: Lex<'a>,
    L: Lex<'a>,
    D: Lex<'a>,
    SH: Lex<'a>,
{
    Incomplete,
    Lexeme(L::Err),
    Delimiter(D::Err),
    EscapedString(ES::Err),
    Shell(SH::Err),
    E(String),
}

pub struct Lexer<'a, ES, L, D, SH> {
    lexable: &'a str,
    last_tok_lexeme: bool,
    _marker: std::marker::PhantomData<(ES, L, D, SH)>,
}
impl<'a, ES, L, D, SH> Lexer<'a, ES, L, D, SH>
where
    ES: Lex<'a>,
    L: Lex<'a>,
    D: Lex<'a>,
    SH: Lex<'a>,
{
    pub fn new(lexable: &'a str) -> Self {
        Self {
            lexable,
            last_tok_lexeme: false,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn lex(&mut self) -> Option<Result<Token<'a, ES, L, D, SH>, LexerError<'a, ES, L, D, SH>>> {
        let res = self._lex();
        if matches!(res, Some(Err(_))) {
            self.lexable = "";
        }
        res
    }
    fn _lex(&mut self) -> Option<Result<Token<'a, ES, L, D, SH>, LexerError<'a, ES, L, D, SH>>> {
        self.lexable = self.lexable.trim_ascii();
        enum State {
            Start,
            Lexeme,
            Delimiter,
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
                State::Start if ch.is_ascii_punctuation() => State::Delimiter,
                State::Start => {
                    return Some(Err(LexerError::E(format!(
                        "unexpected first character {ch}"
                    ))));
                }
                State::Lexeme => {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        continue;
                    } else {
                        let lexeme_string = &self.lexable[..index];
                        self.lexable = &self.lexable[index..];
                        match L::lex(lexeme_string) {
                            Ok(l) => {
                                self.last_tok_lexeme = true;
                                return Some(Ok(Token::Lexeme(l)));
                            }
                            Err(e) => return Some(Err(LexerError::Lexeme(e))),
                        }
                    }
                }
                State::Delimiter => {
                    if ch.is_ascii_punctuation() {
                        continue;
                    } else {
                        let delimiter_string = &self.lexable[..index];
                        self.lexable = &self.lexable[index..];
                        match D::lex(delimiter_string) {
                            Ok(d) => return Some(Ok(Token::Delimiter(d))),
                            Err(e) => return Some(Err(LexerError::Delimiter(e))),
                        }
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
                            return Some(Ok(Token::SInteger(0)));
                        }
                    }
                    ch if ch.is_ascii_digit() => State::NumberDigits,
                    'i' => State::SInteger1,
                    'u' => State::UInteger1,
                    _ => {
                        self.lexable = &self.lexable[index..];
                        return Some(Ok(Token::SInteger(0)));
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
                            let integer_string = &self.lexable[..index];
                            self.lexable = &self.lexable[index..];
                            match integer_string.parse::<i64>() {
                                Ok(n) => return Some(Ok(Token::SInteger(n))),
                                Err(e) => {
                                    return Some(Err(LexerError::E(format!(
                                        "error parsing integer string: {integer_string:?} -> {e:?}"
                                    ))));
                                }
                            }
                        }
                    }
                    'i' => State::SInteger1,
                    'u' => State::UInteger1,
                    _ => {
                        let integer_string = &self.lexable[..index];
                        self.lexable = &self.lexable[index..];
                        match integer_string.parse::<i64>() {
                            Ok(n) => return Some(Ok(Token::SInteger(n))),
                            Err(e) => {
                                return Some(Err(LexerError::E(format!(
                                    "error parsing integer string: {integer_string:?} -> {e:?}"
                                ))));
                            }
                        }
                    }
                },
                State::HexDigits => {
                    if ch.is_ascii_hexdigit() {
                        continue;
                    } else {
                        let hex_string = &self.lexable[2..index];
                        self.lexable = &self.lexable[index..];
                        match u64::from_str_radix(hex_string, 16) {
                            Ok(n) => return Some(Ok(Token::UInteger(n))),
                            Err(e) => {
                                return Some(Err(LexerError::E(format!(
                                    "error parsing hex number string: {hex_string:?} -> {e:?}"
                                ))));
                            }
                        }
                    }
                }
                State::BinaryDigits => {
                    if ch == '0' || ch == '1' {
                        continue;
                    } else {
                        let binary_string = &self.lexable[2..index];
                        self.lexable = &self.lexable[index..];
                        match u64::from_str_radix(binary_string, 2) {
                            Ok(n) => return Some(Ok(Token::UInteger(n))),
                            Err(e) => {
                                return Some(Err(LexerError::E(format!(
                                    "error parsing binary number string: {binary_string:?} -> {e:?}"
                                ))));
                            }
                        }
                    }
                }
                State::FloatDot => State::FloatDigits,
                State::FloatDigits => {
                    if ch.is_ascii_digit() {
                        continue;
                    } else {
                        let float_string = &self.lexable[..index];
                        self.lexable = &self.lexable[index..];
                        match float_string.parse::<f64>() {
                            Ok(n) => return Some(Ok(Token::Float(n))),
                            Err(e) => {
                                return Some(Err(LexerError::E(format!(
                                    "error parsing float string: {float_string:?} -> {e:?}"
                                ))));
                            }
                        }
                    }
                }
                State::SInteger1 if ch == '6' => State::SInteger2,
                State::SInteger1 => {
                    return Some(Err(LexerError::E(format!("expected 6 got {ch}"))));
                }
                State::SInteger2 if ch == '4' => {
                    let integer_string = &self.lexable[..index - 2];
                    self.lexable = &self.lexable[index + 1..];
                    match integer_string.parse::<i64>() {
                        Ok(n) => return Some(Ok(Token::SInteger(n))),
                        Err(e) => {
                            return Some(Err(LexerError::E(format!(
                                "error parsing integer string: {integer_string:?} -> {e:?}"
                            ))));
                        }
                    }
                }
                State::SInteger2 => {
                    return Some(Err(LexerError::E(format!("expected 4 got {ch}"))));
                }
                State::UInteger1 if ch == '6' => State::UInteger2,
                State::UInteger1 => {
                    return Some(Err(LexerError::E(format!("expected 6 got {ch}"))));
                }
                State::UInteger2 if ch == '4' => {
                    let integer_string = &self.lexable[..index - 2];
                    self.lexable = &self.lexable[index + 1..];
                    match integer_string.parse::<u64>() {
                        Ok(n) => return Some(Ok(Token::UInteger(n))),
                        Err(e) => {
                            return Some(Err(LexerError::E(format!(
                                "error parsing integer string: {integer_string:?} -> {e:?}"
                            ))));
                        }
                    }
                }
                State::UInteger2 => {
                    return Some(Err(LexerError::E(format!("expected 4 got {ch}"))));
                }
                State::E if ch.is_ascii_alphanumeric() || ch == '_' => State::Lexeme,
                State::E if ch == '#' => todo!(),
                State::E if ch == '"' => todo!(),
                State::E => {
                    let lexeme_string = &self.lexable[..index];
                    self.lexable = &self.lexable[index..];
                    match L::lex(lexeme_string) {
                        Ok(l) => {
                            self.last_tok_lexeme = true;
                            return Some(Ok(Token::Lexeme(l)));
                        }
                        Err(e) => return Some(Err(LexerError::Lexeme(e))),
                    }
                }
                State::RawStringPrefix { prefix } if ch == '#' => {
                    State::RawStringPrefix { prefix: prefix + 1 }
                }
                State::RawStringPrefix { prefix } if ch == '"' => State::RawString { prefix },
                State::RawStringPrefix { prefix: _ } => {
                    return Some(Err(LexerError::E(format!(
                        "expected # or \" but got {ch} at byte index {index}"
                    ))));
                }
                State::RawString { prefix } if ch == '"' => {
                    if prefix == 0 {
                        let token_end = index + '"'.len_utf8();
                        let delim_bytes = '"'.len_utf8();
                        let raw_string_string = &self.lexable[delim_bytes..token_end - delim_bytes];
                        self.lexable = &self.lexable[token_end..];
                        return Some(Ok(Token::RawString(raw_string_string)));
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
                        return Some(Ok(Token::RawString(raw_string_string)));
                    } else {
                        State::RawStringSuffix { prefix, suffix }
                    }
                }
                State::RawStringSuffix { prefix, suffix: _ } => State::RawString { prefix },
                State::Shell => {
                    if ch != ';' {
                        continue;
                    } else {
                        let shell_string = &self.lexable[1..index];
                        self.lexable = &self.lexable[index..];
                        match SH::lex(shell_string) {
                            Ok(sh) => {
                                self.last_tok_lexeme = true;
                                return Some(Ok(Token::Shell(sh)));
                            }
                            Err(e) => return Some(Err(LexerError::Shell(e))),
                        }
                    }
                }
            };
        }
        match state {
            State::Start => None,
            State::Delimiter => {
                let delimiter_string = self.lexable;
                self.lexable = "";
                match D::lex(delimiter_string) {
                    Ok(d) => return Some(Ok(Token::Delimiter(d))),
                    Err(e) => return Some(Err(LexerError::Delimiter(e))),
                }
            }
            State::E | State::Lexeme => {
                let lexeme_string = self.lexable;
                self.lexable = "";
                match L::lex(lexeme_string) {
                    Ok(l) => {
                        self.last_tok_lexeme = true;
                        return Some(Ok(Token::Lexeme(l)));
                    }
                    Err(e) => return Some(Err(LexerError::Lexeme(e))),
                }
            }
            State::NumberZero => {
                self.lexable = "";
                return Some(Ok(Token::SInteger(0)));
            }
            State::NumberDigits => {
                let integer_string = self.lexable;
                self.lexable = "";
                match integer_string.parse::<i64>() {
                    Ok(n) => return Some(Ok(Token::SInteger(n))),
                    Err(e) => {
                        return Some(Err(LexerError::E(format!(
                            "error parsing integer string: {integer_string:?} -> {e:?}"
                        ))));
                    }
                }
            }
            State::HexDigits => {
                let hex_string = &self.lexable[2..];
                self.lexable = "";
                match u64::from_str_radix(hex_string, 16) {
                    Ok(n) => return Some(Ok(Token::UInteger(n))),
                    Err(e) => {
                        return Some(Err(LexerError::E(format!(
                            "error parsing hex number string: {hex_string:?} -> {e:?}"
                        ))));
                    }
                }
            }
            State::BinaryDigits => {
                let binary_string = &self.lexable[2..];
                self.lexable = "";
                match u64::from_str_radix(binary_string, 2) {
                    Ok(n) => return Some(Ok(Token::UInteger(n))),
                    Err(e) => {
                        return Some(Err(LexerError::E(format!(
                            "error parsing binary number string: {binary_string:?} -> {e:?}"
                        ))));
                    }
                }
            }
            State::FloatDot => unreachable!(),
            State::FloatDigits => {
                let float_string = self.lexable;
                self.lexable = "";
                match float_string.parse::<f64>() {
                    Ok(n) => return Some(Ok(Token::Float(n))),
                    Err(e) => {
                        return Some(Err(LexerError::E(format!(
                            "error parsing float string: {float_string:?} -> {e:?}"
                        ))));
                    }
                }
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
            } => Some(Err(LexerError::Incomplete)),
            State::Shell => {
                let shell_string = &self.lexable[1..];
                self.lexable = "";
                match SH::lex(shell_string) {
                    Ok(sh) => {
                        self.last_tok_lexeme = true;
                        return Some(Ok(Token::Shell(sh)));
                    }
                    Err(e) => return Some(Err(LexerError::Shell(e))),
                }
            }
        }
    }
}
impl<'a, ES, L, D, SH> Iterator for Lexer<'a, ES, L, D, SH>
where
    ES: Lex<'a>,
    L: Lex<'a>,
    D: Lex<'a>,
    SH: Lex<'a>,
{
    type Item = Result<Token<'a, ES, L, D, SH>, LexerError<'a, ES, L, D, SH>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

impl<'a, ES, L, D, SH> std::iter::FusedIterator for Lexer<'a, ES, L, D, SH>
where
    ES: Lex<'a>,
    L: Lex<'a>,
    D: Lex<'a>,
    SH: Lex<'a>,
{
}
