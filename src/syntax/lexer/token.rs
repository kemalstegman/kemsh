//! todo: escaped strings

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Token {
    // literals
    Integer(i64),
    Float(f64),
    RawString(String),
    // delimiters
    DELIM_EXCLAMATIONMARK,         // !
    DELIM_AMPERSAND,               // &
    DELIM_DOUBLEAMPERSAND,         // &&
    DELIM_ASTERISK,                // *
    DELIM_PERCENT,                 // %
    DELIM_OPENPARENTHESIS,         // (
    DELIM_CLOSEPARENTHESIS,        // )
    DELIM_MINUS,                   // -
    DELIM_PLUS,                    // +
    DELIM_EQUAL,                   // =
    DELIM_DOUBLEEQUAL,             // ==
    DELIM_PIPE,                    // |
    DELIM_DOUBLEPIPE,              // ||
    DELIM_OPENBRACKET,             // [
    DELIM_CLOSEBRACKET,            // ]
    DELIM_HASHTAGOPENBRACE,        // #{
    DELIM_OPENBRACE,               // {
    DELIM_CLOSEBRACE,              // }
    DELIM_CLOSEBRACEHASHTAG,       // }#
    DELIM_SEMICOLON,               // ;
    DELIM_COLON,                   // :
    DELIM_OPENANGLEBRACKET,        // <
    DELIM_CLOSEANGLEBRACKET,       // >
    DELIM_DOUBLECLOSEANGLEBRACKET, // >>
    DELIM_COMMA,                   // ,
    DELIM_PERIOD,                  // .
    DELIM_FORWARDSLASH,            // /
    // unreserved lexeme
    UnreservedLexeme(String),
    // reserved lexemes
    LEX_TRUE,
    LEX_FALSE,
    LEX_LET,
    LEX_FOR,
    LEX_WHILE,
    LEX_LOOP,
    LEX_FN,
    LEX_RETURN,
    LEX_BREAK,
    LEX_RUN,
    LEX_SPAWN,
    LEX_WAIT,
    LEX_ECHO,
    LEX_ECHON,
    LEX_CD,
    LEX_EXIT,
    LEX_INTEGER,
    LEX_FLOAT,
    LEX_STRING,
    LEX_BOOLEAN,
    LEX_OPTION,
    LEX_RESULT,
    LEX_LIST,
    LEX_MAP,
    LEX_FUNCTION,
    LEX_JCMD,
    LEX_JHANDLE,
    LEX_JEXIT,
    LEX_FHANDLE,
    LEX_VOID,
}

impl Token {
    pub fn is_lexeme(&self) -> bool {
        self.get_lexeme().is_some()
    }
    pub fn get_lexeme(&self) -> Option<&str> {
        match self {
            Self::UnreservedLexeme(lexeme) => Some(lexeme.as_str()),
            Self::LEX_TRUE => Some("true"),
            Self::LEX_FALSE => Some("false"),
            Self::LEX_LET => Some("let"),
            Self::LEX_FOR => Some("for"),
            Self::LEX_WHILE => Some("while"),
            Self::LEX_LOOP => Some("loop"),
            Self::LEX_RETURN => Some("return"),
            Self::LEX_BREAK => Some("break"),
            Self::LEX_RUN => Some("run"),
            Self::LEX_SPAWN => Some("spawn"),
            Self::LEX_WAIT => Some("wait"),
            Self::LEX_ECHO => Some("echo"),
            Self::LEX_ECHON => Some("echon"),
            Self::LEX_CD => Some("cd"),
            Self::LEX_EXIT => Some("exit"),
            Self::LEX_INTEGER => Some("integer"),
            Self::LEX_FLOAT => Some("float"),
            Self::LEX_STRING => Some("string"),
            Self::LEX_BOOLEAN => Some("boolean"),
            Self::LEX_OPTION => Some("option"),
            Self::LEX_RESULT => Some("result"),
            Self::LEX_LIST => Some("list"),
            Self::LEX_MAP => Some("map"),
            Self::LEX_JCMD => Some("jcmd"),
            Self::LEX_JHANDLE => Some("jhandle"),
            Self::LEX_JEXIT => Some("jexit"),
            Self::LEX_FHANDLE => Some("fhandle"),
            Self::LEX_VOID => Some("void"),
            _ => None,
        }
    }
    pub fn lexeme_from_string(string: String) -> Self {
        match string.as_str() {
            "true" => Self::LEX_TRUE,
            "false" => Self::LEX_FALSE,
            "let" => Self::LEX_LET,
            "for" => Self::LEX_FOR,
            "while" => Self::LEX_WHILE,
            "loop" => Self::LEX_LOOP,
            "return" => Self::LEX_RETURN,
            "break" => Self::LEX_BREAK,
            "run" => Self::LEX_RUN,
            "spawn" => Self::LEX_SPAWN,
            "wait" => Self::LEX_WAIT,
            "echo" => Self::LEX_ECHO,
            "echon" => Self::LEX_ECHON,
            "fn" => Self::LEX_FN,
            "cd" => Self::LEX_CD,
            "exit" => Self::LEX_EXIT,
            "integer" => Self::LEX_INTEGER,
            "float" => Self::LEX_FLOAT,
            "string" => Self::LEX_STRING,
            "boolean" => Self::LEX_BOOLEAN,
            "option" => Self::LEX_OPTION,
            "result" => Self::LEX_RESULT,
            "list" => Self::LEX_LIST,
            "map" => Self::LEX_MAP,
            "function" => Self::LEX_FUNCTION,
            "jcmd" => Self::LEX_JCMD,
            "jhandle" => Self::LEX_JHANDLE,
            "jexit" => Self::LEX_JEXIT,
            "fhandle" => Self::LEX_FHANDLE,
            "void" => Self::LEX_VOID,
            _ => Self::UnreservedLexeme(string),
        }
    }
    pub fn is_delimiter(&self) -> bool {
        match self {
            Self::DELIM_EXCLAMATIONMARK => true,
            Self::DELIM_AMPERSAND => true,
            Self::DELIM_DOUBLEAMPERSAND => true,
            Self::DELIM_ASTERISK => true,
            Self::DELIM_PERCENT => true,
            Self::DELIM_OPENPARENTHESIS => true,
            Self::DELIM_CLOSEPARENTHESIS => true,
            Self::DELIM_MINUS => true,
            Self::DELIM_PLUS => true,
            Self::DELIM_EQUAL => true,
            Self::DELIM_DOUBLEEQUAL => true,
            Self::DELIM_PIPE => true,
            Self::DELIM_DOUBLEPIPE => true,
            Self::DELIM_OPENBRACKET => true,
            Self::DELIM_CLOSEBRACKET => true,
            Self::DELIM_HASHTAGOPENBRACE => true,
            Self::DELIM_OPENBRACE => true,
            Self::DELIM_CLOSEBRACE => true,
            Self::DELIM_CLOSEBRACEHASHTAG => true,
            Self::DELIM_SEMICOLON => true,
            Self::DELIM_COLON => true,
            Self::DELIM_OPENANGLEBRACKET => true,
            Self::DELIM_CLOSEANGLEBRACKET => true,
            Self::DELIM_DOUBLECLOSEANGLEBRACKET => true,
            Self::DELIM_COMMA => true,
            Self::DELIM_PERIOD => true,
            Self::DELIM_FORWARDSLASH => true,
            _ => false,
        }
    }
}
