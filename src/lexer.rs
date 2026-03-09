use std::iter::Peekable;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Keyword(TokenKeyword),
    VariableName(String),
    Number(TokenNumber),
    String(String),
    Boolean(bool),
    Delimeter(TokenDelimeter),
}

#[derive(Debug)]
pub enum TokenKeyword {
    Let,
    Echo,
    For,
    While,
    Loop,
    Return,
    Break,
    Run,
    Spawn,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TokenNumber {
    pub string: String,
}

#[derive(Debug)]
pub enum TokenDelimeter {
    Whitespace,
    ExclamationMark,
    // At,
    // Hashtag,
    // Dollar,
    // Percent,
    Carret,
    Ampersand,
    AmpersandAmpersand,
    Asterisk,
    OpenParenthesis,
    CloseParenthesis,
    Minus,
    Plus,
    Equal,
    EqualEqual,
    Pipe,
    PipePipe,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Colon,
    OpenAngleBracket,
    OpenAngleBracketOpenAngleBracket,
    Comma,
    CloseAngleBracket,
    CloseAngleBracketCloseAngleBracket,
    Period,
    FowardSlash,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LexError {
    message: String,
}

pub fn lex(
    mut input_stream: Peekable<impl Iterator<Item = char>>,
) -> Result<Option<Vec<Token>>, LexError> {
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(ch) = input_stream.next() {
        if let ' ' | '\t' | '\n' | '\r' = ch {
            // match tokens.last() {
            //     Some(Token::Delimeter(TokenDelimeter::Whitespace)) => (),
            //     _ => {
            //         tokens.push(Token::Delimeter(TokenDelimeter::Whitespace));
            //     }
            // }
            continue;
        } else if let 'a'..='z' | 'A'..='Z' | '_' = ch {
            let mut identifier = String::from(ch);
            while let Some(ch) = input_stream.next_if(|ch| match ch {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
                _ => false,
            }) {
                identifier.push(ch);
            }
            tokens.push(match identifier.as_str() {
                "let" => Token::Keyword(TokenKeyword::Let),
                "for" => Token::Keyword(TokenKeyword::For),
                "while" => Token::Keyword(TokenKeyword::While),
                "loop" => Token::Keyword(TokenKeyword::Loop),
                "return" => Token::Keyword(TokenKeyword::Return),
                "break" => Token::Keyword(TokenKeyword::Break),
                "run" => Token::Keyword(TokenKeyword::Run),
                "spawn" => Token::Keyword(TokenKeyword::Spawn),
                "echo" => Token::Keyword(TokenKeyword::Echo),
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                _ => Token::VariableName(identifier),
            });
        } else if let '0'..='9' = ch {
            let mut number_string = String::from(ch);
            while let Some(ch) = input_stream.next_if(|ch| match ch {
                '0'..='9' => true,
                _ => false,
            }) {
                number_string.push(ch);
            }
            tokens.push(Token::Number(TokenNumber {
                string: number_string,
            }));
        } else if let '#' | '"' = ch {
            // todo!()
            let _is_escaped = match tokens.last() {
                None => false,
                Some(Token::VariableName(s)) if s == "e" => {
                    tokens.pop();
                    true
                }
                Some(_) => false,
            };
            let hashtag_count = match ch {
                '"' => 0,
                '#' => {
                    let mut count = 1;
                    while let Some(_) = input_stream.next_if_eq(&'#') {
                        count += 1;
                    }
                    match input_stream.next() {
                        None => return Ok(None),
                        Some('"') => (),
                        Some(_) => {
                            return Err(LexError {
                                message: format!("Expected quotation mark"),
                            });
                        }
                    }
                    count
                }
                _ => unreachable!(),
            };
            let mut string = String::new();
            'string: loop {
                match input_stream.next() {
                    None => return Ok(None),
                    Some('"') => {
                        for i in 0..hashtag_count {
                            match input_stream.next() {
                                None => return Ok(None),
                                Some('#') => (),
                                Some(ch) => {
                                    string.push_str(&format!("\"{}{ch}", "#".repeat(i)));
                                    continue 'string;
                                }
                            }
                        }
                        break 'string;
                    }
                    Some(ch) => string.push(ch),
                }
            }
            tokens.push(Token::String(string));
        } else if let Some(delim) = match ch {
            // '#' => Some(TokenDelimeter::Hashtag),
            '^' => Some(TokenDelimeter::Carret),
            '&' => match input_stream.next_if_eq(&'&') {
                Some(_) => Some(TokenDelimeter::AmpersandAmpersand),
                None => Some(TokenDelimeter::Ampersand),
            },
            '*' => Some(TokenDelimeter::Asterisk),
            '(' => Some(TokenDelimeter::OpenParenthesis),
            ')' => Some(TokenDelimeter::CloseParenthesis),
            '-' => Some(TokenDelimeter::Minus),
            '+' => Some(TokenDelimeter::Plus),
            '=' => match input_stream.next_if_eq(&'=') {
                Some(_) => Some(TokenDelimeter::EqualEqual),
                None => Some(TokenDelimeter::Equal),
            },
            '/' => Some(TokenDelimeter::FowardSlash),
            '|' => match input_stream.next_if_eq(&'|') {
                Some(_) => Some(TokenDelimeter::PipePipe),
                None => Some(TokenDelimeter::Pipe),
            },
            '[' => Some(TokenDelimeter::OpenBracket),
            ']' => Some(TokenDelimeter::CloseBracket),
            '{' => Some(TokenDelimeter::OpenBrace),
            '}' => Some(TokenDelimeter::CloseBrace),
            ';' => Some(TokenDelimeter::Semicolon),
            ':' => Some(TokenDelimeter::Colon),
            ',' => Some(TokenDelimeter::Comma),
            '<' => match input_stream.next_if_eq(&'<') {
                Some(_) => Some(TokenDelimeter::OpenAngleBracketOpenAngleBracket),
                None => Some(TokenDelimeter::OpenAngleBracket),
            },
            '>' => match input_stream.next_if_eq(&'>') {
                Some(_) => Some(TokenDelimeter::CloseAngleBracketCloseAngleBracket),
                None => Some(TokenDelimeter::CloseAngleBracket),
            },
            '.' => Some(TokenDelimeter::Period),
            _ => None,
        } {
            tokens.push(Token::Delimeter(delim));
        } else {
            return Err(LexError {
                message: format!("Unknown character: {ch:?}"),
            });
        }
    }
    Ok(Some(tokens))
}
