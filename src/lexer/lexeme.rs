#[derive(Debug, PartialEq, Eq)]
pub enum Lexeme<'a> {
    LexLet,
    LexUnit,
    LexI64,
    LexU64,
    LexBool,
    LexTrue,
    LexFalse,
    LexString,
    LexList,
    LexMap,
    LexOption,
    LexSome,
    LexNone,
    LexResult,
    LexOk,
    LexErr,
    LexFn,
    LexLoop,
    LexWhile,
    LexFor,
    LexIf,
    LexJcmd,
    LexJhandle,
    LexJexit,
    LexProcess,
    LexRun,
    LexSpawn,
    LexWait,
    LexFhandle,
    LexDrop,
    LexCd,
    Identifier(&'a str),
}

impl<'a> From<&'a str> for Lexeme<'a> {
    fn from(lexeme: &'a str) -> Self {
        match lexeme {
            "let" => Self::LexLet,
            "unit" => Self::LexUnit,
            "i64" => Self::LexI64,
            "u64" => Self::LexU64,
            "bool" => Self::LexBool,
            "true" => Self::LexTrue,
            "false" => Self::LexFalse,
            "string" => Self::LexString,
            "list" => Self::LexList,
            "map" => Self::LexMap,
            "option" => Self::LexOption,
            "some" => Self::LexSome,
            "none" => Self::LexNone,
            "result" => Self::LexResult,
            "ok" => Self::LexOk,
            "err" => Self::LexErr,
            "fn" => Self::LexFn,
            "loop" => Self::LexLoop,
            "while" => Self::LexWhile,
            "for" => Self::LexFor,
            "if" => Self::LexIf,
            "jcmd" => Self::LexJcmd,
            "jhandle" => Self::LexJhandle,
            "jexit" => Self::LexJexit,
            "process" => Self::LexProcess,
            "run" => Self::LexRun,
            "spawn" => Self::LexSpawn,
            "wait" => Self::LexWait,
            "fhandle" => Self::LexFhandle,
            "drop" => Self::LexDrop,
            "cd" => Self::LexCd,
            lexeme => Self::Identifier(lexeme),
        }
    }
}

/// Finds the byte index after the lexeme that starts at the beginning of the string.
/// Skips the first `skip_bytes` of the string before doing checks.
fn assume_lexeme_end_index(lexable: &str, skip_bytes: usize) -> usize {
    lexable[skip_bytes..]
        .char_indices()
        .find(|&(_, c)| !(c.is_ascii_alphanumeric() || c == '_'))
        .map_or(lexable.len(), |(i, _)| i + skip_bytes)
}

impl<'a> Lexeme<'a> {
    /// Assumes the first char is one byte, and the first char is a valid lexeme.
    /// Lexes a lexeme from a lexable mutable immutable string slice. Mutates the
    /// lexable slice so the characters of the returned lexeme are removed from the
    /// slice.
    pub fn lex_assume_first_byte<'b>(lexable: &mut &'b str) -> Lexeme<'b> {
        let index = assume_lexeme_end_index(lexable, 1);
        let lexeme = &lexable[..index];
        *lexable = &lexable[index..];
        Lexeme::from(lexeme)
    }
}
