#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Delimiter {
    // single character
    OpenBrace,          // {
    CloseBrace,         // }
    OpenSquareBracket,  // [
    CloseSquareBracket, // ]
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    OpenAngleBracket,   // <
    CloseAngleBracket,  // >
    Comma,              // ,
    Period,             // .
    Semicolon,          // ;
    Colon,              // :
    HyphenMinus,        // -
    Plus,               // +
    Equal,              // =
    ForwardSlash,       // /
    BackSlash,          // \
    Asterisk,           // *
    Bang,               // !
    // multi-character
    OpenBraceBracket,  // {[
    CloseBracketBrace, // ]}
    EqualEqual,        // ==
}
