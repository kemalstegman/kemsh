use crate::ast::ValueKind;

type Identifier = String;

#[derive(Debug)]
pub enum AST2 {
    Identifier(Identifier),
    Value(Value2),
    Operation(Box<Operation2>),
    BOperation(Box<BOperation2>),
    VOperation(VOperation2),
    Control(Control2),
}

#[derive(Debug)]
pub enum Control2 {
    Loop(Vec<AST2>),
    While((), Vec<AST2>),
    For((), Vec<AST2>),
    If(Box<(AST2, AST2)>),
    IfElse(Box<(AST2, AST2, AST2)>),
}

#[derive(Debug)]
pub enum Value2 {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(String),
    UNIT,
    Option(Option<Box<Value2>>),
    Result(Result<Box<Value2>, Box<Value2>>),
    List(Vec<Value2>),
    Map(Vec<(Identifier, Value2)>),
    Function(Vec<(Identifier,)>, Vec<AST2>),
    Jcmd(()),
    Jhandle(()),
    Jexit(()),
    Fhandle(()),
}

impl Value2 {
    pub fn kind(&self) -> ValueKind {
        match self {
            Value2::I64(_) => ValueKind::I64,
            Value2::U64(_) => ValueKind::U64,
            Value2::F64(_) => ValueKind::F64,
            Value2::BOOL(_) => ValueKind::BOOL,
            Value2::STRING(_) => ValueKind::STRING,
            Value2::UNIT => ValueKind::UNIT,
            Value2::Option(_) => ValueKind::Option,
            Value2::Result(_) => ValueKind::Result,
            Value2::List(_) => ValueKind::List,
            Value2::Map(_) => ValueKind::Map,
            Value2::Function(_, _) => ValueKind::Function,
            Value2::Jcmd(_) => ValueKind::Jcmd,
            Value2::Jhandle(_) => ValueKind::Jhandle,
            Value2::Jexit(_) => ValueKind::Jexit,
            Value2::Fhandle(_) => ValueKind::Fhandle,
        }
    }
}

#[derive(Debug)]
pub enum BOperation2 {
    Add {
        lhs: AST2,
        rhs: AST2,
    },
    Subtract {
        lhs: AST2,
        rhs: AST2,
    },
    Multiply {
        lhs: AST2,
        rhs: AST2,
    },
    Divide {
        lhs: AST2,
        rhs: AST2,
    },
    Negate {
        rhs: AST2,
    },
    Some {
        rhs: AST2,
    },
    Ok {
        rhs: AST2,
    },
    Err {
        rhs: AST2,
    },
    ChangeDirectory {
        rhs: AST2,
    },
    Run {
        rhs: AST2,
    },
    Spawn {
        rhs: AST2,
    },
    Process {
        rhs: AST2,
    },
    Wait {
        rhs: AST2,
    },
    Drop {
        identifier: Identifier,
    },
    Let {
        identifier: Identifier,
        r#type: Option<ValueKind>,
        expression: Option<AST2>,
    },
    Assign(()),
    LessThan(AST2, AST2),
    GreaterThan(AST2, AST2),
    Equal(AST2, AST2),
    Index(AST2, AST2),
    Shell(String),
}

#[derive(Debug)]
pub enum VOperation2 {
    Call(Box<AST2>, Vec<AST2>),
    List(Vec<AST2>),
    Map(Vec<(String, AST2)>),
}
