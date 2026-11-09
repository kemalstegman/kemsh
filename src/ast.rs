use std::rc::Rc;

pub trait TextOwnership {
    type Text<'a>;
}

pub struct SourceBorrowedText;

impl TextOwnership for SourceBorrowedText {
    type Text<'a> = &'a str;
}

pub struct OwnedText;

impl TextOwnership for OwnedText {
    type Text<'a> = String;
}

pub trait ValueOwnership {
    type DirectValue<V>;
    type IndirectValue<V>;
}

pub struct OwnedValues;

impl ValueOwnership for OwnedValues {
    type DirectValue<V> = V;
    type IndirectValue<V> = Box<V>;
}

pub struct BorrowedValues;

impl ValueOwnership for BorrowedValues {
    type DirectValue<V> = Rc<V>;
    type IndirectValue<V> = Rc<V>;
}

#[derive(Debug)]
pub enum AST<'a, TO: TextOwnership, VO: ValueOwnership> {
    Identifier(TO::Text<'a>),
    Value(VO::DirectValue<Value<'a, TO, VO>>),
    POperation(Box<POperation<Self>>),
    IOperation(Box<IOperation<Self>>),
    SpecialOperation(SpecialOperation<'a, TO, Self, ValueKind>),
}

#[derive(Debug)]
pub enum Value<'a, TO: TextOwnership, VO: ValueOwnership> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(TO::Text<'a>),
    UNIT,
    Option(Option<VO::IndirectValue<Self>>),
    Result(Result<VO::IndirectValue<Self>, VO::IndirectValue<Self>>),
    List(Vec<VO::DirectValue<Self>>),
    Map(Vec<(TO::Text<'a>, VO::DirectValue<Self>)>),
    Function(Box<(Vec<(TO::Text<'a>,)>, Vec<AST<'a, TO, VO>>)>),
    Jcmd(()),
    Jhandle(()),
    Jexit(()),
    Fhandle(()),
}

impl<'a, TO: TextOwnership, VO: ValueOwnership> Value<'a, TO, VO> {
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
