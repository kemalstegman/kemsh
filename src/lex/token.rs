#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    VariableName(String),
    Keyword(TokenKeyword),
    Boolean(bool),
    String(String),
    Number(i64),
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
            "true" => Self::Boolean(true),
            "false" => Self::Boolean(false),
            _ => Self::VariableName(string),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenDelimeter {
    Whitespace,
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
