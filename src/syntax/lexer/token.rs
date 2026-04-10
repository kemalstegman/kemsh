//! todo: escaped strings

#[derive(Debug)]
pub enum Token {
    // lexemes
    Reserved(ReservedLexeme),
    Unreserved(Lexeme),
    // literals
    Integer(i64),
    Float(f64),
    LiteralString(String),
    // delimeters
    Delimeter(Delimeter),
}

impl Token {
    pub fn from_lexeme(string: String) -> Self {
        match ReservedLexeme::try_from(string.as_str()) {
            Ok(reserved) => Self::Reserved(reserved),
            Err(()) => Self::Unreserved(string),
        }
    }
}

pub type Lexeme = String;

#[derive(Debug)]
pub enum ReservedLexeme {
    True,
    False,
    Let,
    For,
    While,
    Loop,
    Return,
    Break,
    Block,
    Spawn,
    Process,
    Echo,
    Cd,
    Exit,
    Integer,
    Float,
    String,
    Boolean,
    Option,
    Result,
    List,
    Map,
    PCmd,
    PHandle,
    PExit,
    FHandle,
    Void,
}

impl TryFrom<&str> for ReservedLexeme {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            "let" => Ok(Self::Let),
            "for" => Ok(Self::For),
            "while" => Ok(Self::While),
            "loop" => Ok(Self::Loop),
            "return" => Ok(Self::Return),
            "break" => Ok(Self::Break),
            "block" => Ok(Self::Block),
            "spawn" => Ok(Self::Spawn),
            "process" => Ok(Self::Process),
            "echo" => Ok(Self::Echo),
            "cd" => Ok(Self::Cd),
            "exit" => Ok(Self::Exit),
            "integer" => Ok(Self::Integer),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            "boolean" => Ok(Self::Boolean),
            "option" => Ok(Self::Option),
            "result" => Ok(Self::Result),
            "list" => Ok(Self::List),
            "map" => Ok(Self::Map),
            "pcmd" => Ok(Self::PCmd),
            "phandle" => Ok(Self::PHandle),
            "pexit" => Ok(Self::PExit),
            "fhandle" => Ok(Self::FHandle),
            "void" => Ok(Self::Void),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Delimeter {
    ExclamationMark,   // !
    Carret,            // ^
    Ampersand,         // &
    Asterisk,          // *
    Percent,           // %
    OpenParenthesis,   // (
    CloseParenthesis,  // )
    Minus,             // -
    Plus,              // +
    Equal,             // =
    EqualEqual,        // ==
    Pipe,              // |
    OpenBracket,       // [
    CloseBracket,      // ]
    OpenBrace,         // {
    CloseBrace,        // }
    Semicolon,         // ;
    Colon,             // :
    OpenAngleBracket,  // <
    CloseAngleBracket, // >
    Comma,             // ,
    Period,            // .
    ForwardSlash,      // /
}
