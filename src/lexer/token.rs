#[derive(Debug)]
pub enum Token {
    Identifier(String),
    String(String),
    Number(i64),
    Delimeter(Delimeter),
}

#[derive(Debug)]
pub enum Delimeter {
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
