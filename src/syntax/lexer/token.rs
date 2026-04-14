//! todo: escaped strings

#[derive(Debug)]
pub enum Token {
    // lexemes
    Reserved(ReservedLexeme),
    Unreserved(String),
    // literals
    Integer(i64),
    Float(f64),
    RawString(String),
    // delimiters
    Delimiter(Delimiter),
}

impl Token {
    pub fn from_lexeme(string: String) -> Self {
        match ReservedLexeme::try_from(string.as_str()) {
            Ok(reserved) => Self::Reserved(reserved),
            Err(()) => Self::Unreserved(string),
        }
    }
}

#[derive(Debug)]
pub enum ReservedLexeme {
    TRUE,
    FALSE,
    LET,
    FOR,
    WHILE,
    LOOP,
    RETURN,
    BREAK,
    RUN,
    SPAWN,
    WAIT,
    ECHO,
    ECHON,
    CD,
    EXIT,
    INTEGER,
    FLOAT,
    STRING,
    BOOLEAN,
    OPTION,
    RESULT,
    LIST,
    MAP,
    JCMD,
    JHANDLE,
    JEXIT,
    FHANDLE,
    VOID,
}

impl TryFrom<&str> for ReservedLexeme {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "true" => Ok(ReservedLexeme::TRUE),
            "false" => Ok(ReservedLexeme::FALSE),
            "let" => Ok(ReservedLexeme::LET),
            "for" => Ok(ReservedLexeme::FOR),
            "while" => Ok(ReservedLexeme::WHILE),
            "loop" => Ok(ReservedLexeme::LOOP),
            "return" => Ok(ReservedLexeme::RETURN),
            "break" => Ok(ReservedLexeme::BREAK),
            "run" => Ok(ReservedLexeme::RUN),
            "spawn" => Ok(ReservedLexeme::SPAWN),
            "wait" => Ok(ReservedLexeme::WAIT),
            "echo" => Ok(ReservedLexeme::ECHO),
            "echon" => Ok(ReservedLexeme::ECHON),
            "cd" => Ok(ReservedLexeme::CD),
            "exit" => Ok(ReservedLexeme::EXIT),
            "integer" => Ok(ReservedLexeme::INTEGER),
            "float" => Ok(ReservedLexeme::FLOAT),
            "string" => Ok(ReservedLexeme::STRING),
            "boolean" => Ok(ReservedLexeme::BOOLEAN),
            "option" => Ok(ReservedLexeme::OPTION),
            "result" => Ok(ReservedLexeme::RESULT),
            "list" => Ok(ReservedLexeme::LIST),
            "map" => Ok(ReservedLexeme::MAP),
            "jcmd" => Ok(ReservedLexeme::JCMD),
            "jhandle" => Ok(ReservedLexeme::JHANDLE),
            "jexit" => Ok(ReservedLexeme::JEXIT),
            "fhandle" => Ok(ReservedLexeme::FHANDLE),
            "void" => Ok(ReservedLexeme::VOID),
            _ => Err(()),
        }
    }
}

impl From<ReservedLexeme> for &str {
    fn from(l: ReservedLexeme) -> Self {
        match l {
            ReservedLexeme::TRUE => "true",
            ReservedLexeme::FALSE => "false",
            ReservedLexeme::LET => "let",
            ReservedLexeme::FOR => "for",
            ReservedLexeme::WHILE => "while",
            ReservedLexeme::LOOP => "loop",
            ReservedLexeme::RETURN => "return",
            ReservedLexeme::BREAK => "break",
            ReservedLexeme::RUN => "run",
            ReservedLexeme::SPAWN => "spawn",
            ReservedLexeme::WAIT => "wait",
            ReservedLexeme::ECHO => "echo",
            ReservedLexeme::ECHON => "echon",
            ReservedLexeme::CD => "cd",
            ReservedLexeme::EXIT => "exit",
            ReservedLexeme::INTEGER => "integer",
            ReservedLexeme::FLOAT => "float",
            ReservedLexeme::STRING => "string",
            ReservedLexeme::BOOLEAN => "boolean",
            ReservedLexeme::OPTION => "option",
            ReservedLexeme::RESULT => "result",
            ReservedLexeme::LIST => "list",
            ReservedLexeme::MAP => "map",
            ReservedLexeme::JCMD => "jcmd",
            ReservedLexeme::JHANDLE => "jhandle",
            ReservedLexeme::JEXIT => "jexit",
            ReservedLexeme::FHANDLE => "fhandle",
            ReservedLexeme::VOID => "void",
        }
    }
}

#[derive(Debug)]
pub enum Delimiter {
    ExclamationMark,         // !
    Ampersand,               // &
    DoubleAmpersand,         // &&
    Asterisk,                // *
    Percent,                 // %
    OpenParenthesis,         // (
    CloseParenthesis,        // )
    Minus,                   // -
    Plus,                    // +
    Equal,                   // =
    DoubleEqual,             // ==
    Pipe,                    // |
    DoublePipe,              // ||
    OpenBracket,             // [
    CloseBracket,            // ]
    HashtagOpenBrace,        // #{
    OpenBrace,               // {
    CloseBrace,              // }
    CloseBraceHashtag,       // }#
    Semicolon,               // ;
    Colon,                   // :
    OpenAngleBracket,        // <
    CloseAngleBracket,       // >
    DoubleCloseAngleBracket, // >>
    Comma,                   // ,
    Period,                  // .
    ForwardSlash,            // /
}
