#[derive(Debug)]
pub enum AST<'a> {
    Identifier(&'a str),
    Value(Value<'a>),
    BOperation(Box<BOperation<'a>>),
    VOperation(VOperation<'a>),
    Control(Control<'a>),
}

#[derive(Debug)]
pub enum Control<'a> {
    Loop(Vec<AST<'a>>),
    While((), Vec<AST<'a>>),
    For((), Vec<AST<'a>>),
    If(Box<(AST<'a>, AST<'a>)>),
    IfElse(Box<(AST<'a>, AST<'a>, AST<'a>)>),
}

#[derive(Debug)]
pub enum Value<'a> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(&'a str),
    UNIT,
    Option(Option<Box<Value<'a>>>),
    Result(Result<Box<Value<'a>>, Box<Value<'a>>>),
    List(Vec<Value<'a>>),
    Map(Vec<(&'a str, Value<'a>)>),
    Function(Vec<(&'a str,)>, Vec<AST<'a>>),
    Jcmd(()),
    Jhandle(()),
    Jexit(()),
    Fhandle(()),
}

impl<'a> Value<'a> {
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
            Value::Function(_, _) => ValueKind::Function,
            Value::Jcmd(_) => ValueKind::Jcmd,
            Value::Jhandle(_) => ValueKind::Jhandle,
            Value::Jexit(_) => ValueKind::Jexit,
            Value::Fhandle(_) => ValueKind::Fhandle,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug)]
pub enum BOperation<'a> {
    Add(AST<'a>, AST<'a>),
    Subtract(AST<'a>, AST<'a>),
    Multiply(AST<'a>, AST<'a>),
    Divide(AST<'a>, AST<'a>),
    Negate(AST<'a>),
    Some(AST<'a>),
    Ok(AST<'a>),
    Err(AST<'a>),
    ChangeDirectory(AST<'a>),
    Run(AST<'a>),
    Spawn(AST<'a>),
    Process(AST<'a>),
    Wait(AST<'a>),
    Drop {
        identifier: &'a str,
    },
    Let {
        identifier: &'a str,
        r#type: Option<ValueKind>,
        expression: Option<AST<'a>>,
    },
    Assign(()),
    LessThan(AST<'a>, AST<'a>),
    GreaterThan(AST<'a>, AST<'a>),
    Equal(AST<'a>, AST<'a>),
    Index(AST<'a>, AST<'a>),
    Shell(&'a str),
}

#[derive(Debug)]
pub enum VOperation<'a> {
    Call(Box<AST<'a>>, Vec<AST<'a>>),
    List(Vec<AST<'a>>),
    Map(Vec<(&'a str, AST<'a>)>),
}
