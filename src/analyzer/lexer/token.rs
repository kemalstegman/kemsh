#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    VariableKind(TokenVariableKind),
    VariableName(String),
    Keyword(TokenKeyword),
    Boolean(bool),
    String(String),
    Integer(i64),
    Float(f64),
    Delimeter(TokenDelimeter),
}

impl Token {
    pub fn from_lexeme(string: String) -> Self {
        match string.as_str() {
            "let" => Self::Keyword(TokenKeyword::Let),
            "for" => Self::Keyword(TokenKeyword::For),
            "while" => Self::Keyword(TokenKeyword::While),
            "loop" => Self::Keyword(TokenKeyword::Loop),
            "return" => Self::Keyword(TokenKeyword::Return),
            "break" => Self::Keyword(TokenKeyword::Break),
            "run" => Self::Keyword(TokenKeyword::Run),
            "spawn" => Self::Keyword(TokenKeyword::Spawn),
            "echo" => Self::Keyword(TokenKeyword::Echo),
            "cd" => Self::Keyword(TokenKeyword::Cd),
            "exit" => Self::Keyword(TokenKeyword::Exit),
            "true" => Self::Boolean(true),
            "false" => Self::Boolean(false),
            "integer" => Self::VariableKind(TokenVariableKind::Integer),
            "string" => Self::VariableKind(TokenVariableKind::String),
            "bool" => Self::VariableKind(TokenVariableKind::Boolean),
            _ => Self::VariableName(string),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenKeyword {
    Let,
    For,
    While,
    Loop,
    Return,
    Break,
    Run,
    Spawn,
    Echo,
    Cd,
    Exit,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenVariableKind {
    Integer,
    String,
    Boolean,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenDelimeter {
    ExclamationMark,    // !
    Carret,             // ^
    Ampersand,          // &
    AmpersandAmpersand, // &&
    Asterisk,           // *
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    Minus,              // -
    Plus,               // +
    Equal,              // =
    EqualEqual,         // ==
    Pipe,               // |
    PipePipe,           // ||
    OpenBracket,        // [
    CloseBracket,       // ]
    OpenBrace,          // {
    CloseBrace,         // }
    Semicolon,          // ;
    Colon,              // :
    OpenAngleBracket,   // <
    CloseAngleBracket,  // >
    Comma,              // ,
    Period,             // .
    ForwardSlash,       // /
}
