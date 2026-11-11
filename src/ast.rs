use std::fmt::Debug;

pub trait TextOwnership: Debug {
    type Text<'a>: Debug;
}

#[derive(Debug)]
pub struct SourceBorrowedText;

impl TextOwnership for SourceBorrowedText {
    type Text<'a> = &'a str;
}

#[derive(Debug)]
pub struct OwnedText;

impl TextOwnership for OwnedText {
    type Text<'a> = String;
}

#[derive(Debug)]
pub enum AST<'a, TO: TextOwnership> {
    Identifier(TO::Text<'a>),
    Value(Value<'a, TO>),
    POperation(Box<POperation<Self>>),
    IOperation(Box<IOperation<Self>>),
    SpecialOperation(SpecialOperation<'a, TO, Self, ValueKind>),
}

pub enum Value<'a, TO: TextOwnership> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(TO::Text<'a>),
    UNIT,
    Option(Option<Box<Self>>),
    Result(Result<Box<Self>, Box<Self>>),
    List(Vec<Self>),
    Map(Vec<(TO::Text<'a>, Self)>),
    Function(Box<(Vec<(TO::Text<'a>,)>, Vec<AST<'a, TO>>)>),
    Jcmd(()),
    Jhandle(()),
    Jexit(()),
    Fhandle(()),
}

impl<'a, TO: TextOwnership> Debug for Value<'a, TO> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I64(v) => f.debug_tuple("I64").field(v).finish(),
            Value::U64(v) => f.debug_tuple("U64").field(v).finish(),
            Value::F64(v) => f.debug_tuple("F64").field(v).finish(),
            Value::BOOL(v) => f.debug_tuple("BOOL").field(v).finish(),
            Value::STRING(v) => f.debug_tuple("STRING").field(v).finish(),
            Value::UNIT => f.write_str("UNIT"),
            Value::Option(v) => f.debug_tuple("Option").field(v).finish(),
            Value::Result(v) => f.debug_tuple("Result").field(v).finish(),
            Value::List(v) => f.debug_tuple("List").field(v).finish(),
            Value::Map(v) => f.debug_tuple("Map").field(v).finish(),
            Value::Function(v) => f.debug_tuple("Function").field(v).finish(),
            Value::Jcmd(v) => f.debug_tuple("Jcmd").field(v).finish(),
            Value::Jhandle(v) => f.debug_tuple("Jhandle").field(v).finish(),
            Value::Jexit(v) => f.debug_tuple("Jexit").field(v).finish(),
            Value::Fhandle(v) => f.debug_tuple("Fhandle").field(v).finish(),
        }
    }
}

impl<'a, TO: TextOwnership> Value<'a, TO> {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::I64(_) => ValueKind::I64,
            Value::U64(_) => ValueKind::U64,
            Value::F64(_) => ValueKind::F64,
            Value::BOOL(_) => ValueKind::BOOL,
            Value::STRING(_) => ValueKind::STRING,
            Value::UNIT => ValueKind::UNIT,
            Value::Option(_) => ValueKind::Option,
            Value::Result(_) => ValueKind::Result,
            Value::List(_) => ValueKind::List,
            Value::Map(_) => ValueKind::Map,
            Value::Function(_) => ValueKind::Function,
            Value::Jcmd(_) => ValueKind::Jcmd,
            Value::Jhandle(_) => ValueKind::Jhandle,
            Value::Jexit(_) => ValueKind::Jexit,
            Value::Fhandle(_) => ValueKind::Fhandle,
        }
    }
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
pub enum IOperation<AST> {
    Add { lhs: AST, rhs: AST },
    Subtract { lhs: AST, rhs: AST },
    Multiply { lhs: AST, rhs: AST },
    Divide { lhs: AST, rhs: AST },
    LessThan { lhs: AST, rhs: AST },
    GreaterThan { lhs: AST, rhs: AST },
    Equal { lhs: AST, rhs: AST },
    Index { lhs: AST, rhs: AST },
}

/// prefix operation
#[derive(Debug)]
pub enum POperation<AST> {
    Negate { rhs: AST },
    Some { rhs: AST },
    Ok { rhs: AST },
    Err { rhs: AST },
    ChangeDirectory { rhs: AST },
    Run { rhs: AST },
    Spawn { rhs: AST },
    Process { rhs: AST },
    Wait { rhs: AST },
}

#[derive(Debug)]
pub enum SpecialOperation<'a, TO: TextOwnership, AST, VALUEKIND> {
    List(Vec<AST>),
    Map(Vec<(TO::Text<'a>, AST)>),
    Drop {
        rhs_iden: TO::Text<'a>,
    },
    Let {
        identifier: TO::Text<'a>,
        r#type: Option<VALUEKIND>,
        expression: Option<Box<AST>>,
    },
    Assign(()),
    Call {
        lhs: Box<AST>,
        rhs_args: Vec<AST>,
    },
    Shell(TO::Text<'a>),
    Loop {
        body: Vec<AST>,
    },
    While {
        condition: Box<AST>,
        body: Vec<AST>,
    },
    For {
        iterator: (),
        body: Vec<AST>,
    },
    If(Box<(AST, AST)>),
    IfElse(Box<(AST, AST, AST)>),
}
