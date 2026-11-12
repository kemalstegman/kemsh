pub trait TextOwnership: std::fmt::Debug {
    type Text<'a>: std::fmt::Debug;
    fn as_str<'a>(string: &'a Self::Text<'_>) -> &'a str;
}

#[derive(Debug)]
pub struct SourceBorrowedText;

impl TextOwnership for SourceBorrowedText {
    type Text<'a> = &'a str;
    fn as_str<'a>(string: &'a Self::Text<'_>) -> &'a str {
        string
    }
}

#[derive(Debug)]
pub struct OwnedText;

impl TextOwnership for OwnedText {
    type Text<'a> = String;
    fn as_str<'a>(string: &'a Self::Text<'_>) -> &'a str {
        string.as_str()
    }
}

#[derive(Debug)]
pub enum AST<'a, TO: TextOwnership> {
    Identifier(TO::Text<'a>),
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(TO::Text<'a>),
    UNIT,
    NONE,
    POperation(Box<POperation<'a, TO>>),
    IOperation(Box<IOperation<'a, TO>>),
    SpecialOperation(SpecialOperation<'a, TO>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ValueKind {
    I64,
    U64,
    F64,
    BOOL,
    STRING,
    UNIT,
    Option,
    Result,
    List,
    Map,
    Function,
    Shell,
    Jcmd,
    Jhandle,
    Jexit,
    Fhandle,
}

/// infix operation
#[derive(Debug)]
pub enum IOperation<'a, TO: TextOwnership> {
    Add { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    Subtract { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    Multiply { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    Divide { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    LessThan { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    GreaterThan { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    Equal { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
    Index { lhs: AST<'a, TO>, rhs: AST<'a, TO> },
}

/// prefix operation
#[derive(Debug)]
pub enum POperation<'a, TO: TextOwnership> {
    Negate { rhs: AST<'a, TO> },
    Some { rhs: AST<'a, TO> },
    Ok { rhs: AST<'a, TO> },
    Err { rhs: AST<'a, TO> },
    ChangeDirectory { rhs: AST<'a, TO> },
    Run { rhs: AST<'a, TO> },
    Spawn { rhs: AST<'a, TO> },
    Process { rhs: AST<'a, TO> },
    Wait { rhs: AST<'a, TO> },
}

#[derive(Debug)]
pub enum SpecialOperation<'a, TO: TextOwnership> {
    List(Vec<AST<'a, TO>>),
    Map(Vec<(TO::Text<'a>, AST<'a, TO>)>),
    Drop {
        rhs_iden: TO::Text<'a>,
    },
    Let {
        identifier: TO::Text<'a>,
        r#type: Option<ValueKind>,
        expression: Option<Box<AST<'a, TO>>>,
    },
    Assign {
        identifier: TO::Text<'a>,
        expression: Box<AST<'a, TO>>,
    },
    Call {
        lhs: Box<AST<'a, TO>>,
        rhs_args: Vec<AST<'a, TO>>,
    },
    Shell(TO::Text<'a>),
    Loop {
        body: Vec<AST<'a, TO>>,
    },
    While {
        condition: Box<AST<'a, TO>>,
        body: Vec<AST<'a, TO>>,
    },
    For {
        iterator: (),
        body: Vec<AST<'a, TO>>,
    },
    If(Box<(AST<'a, TO>, AST<'a, TO>)>),
    IfElse(Box<(AST<'a, TO>, AST<'a, TO>, AST<'a, TO>)>),
}
