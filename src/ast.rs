#[derive(Debug)]
pub enum AST<'a> {
    Identifier(&'a str),
    Value(Value<'a>),
    Operation(Operation<'a>),
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
    Map(()),
    Function((), Vec<AST<'a>>),
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
}

#[derive(Debug)]
pub enum Operation<'a> {
    Add(Box<(AST<'a>, AST<'a>)>),
    Subtract(Box<(AST<'a>, AST<'a>)>),
    Multiply(Box<(AST<'a>, AST<'a>)>),
    Divide(Box<(AST<'a>, AST<'a>)>),
    Negate(Box<AST<'a>>),
    Some(Box<AST<'a>>),
    Ok(Box<AST<'a>>),
    Err(Box<AST<'a>>),
    ChangeDirectory(Box<AST<'a>>),
    Run(Box<AST<'a>>),
    Spawn(Box<AST<'a>>),
    Process(Box<AST<'a>>),
    Wait(Box<AST<'a>>),
    Drop {
        identifier: &'a str,
    },
    Let {
        identifier: &'a str,
        r#type: Option<ValueKind>,
        expression: Option<Box<AST<'a>>>,
    },
    Assign,
}
