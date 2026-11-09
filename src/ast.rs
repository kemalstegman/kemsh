#[derive(Debug)]
pub enum AST<STRING, IDENTIFIER> {
    Identifier(IDENTIFIER),
    Value(Value<Self, STRING, IDENTIFIER>),
    POperation(Box<POperation<Self>>),
    IOperation(Box<IOperation<Self>>),
    SpecialOperation(SpecialOperation<Self, STRING, IDENTIFIER, ValueKind>),
}

#[derive(Debug)]
pub enum Value<AST, STRING, IDENTIFIER> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(STRING),
    UNIT,
    Option(Option<Box<Self>>),
    Result(Result<Box<Self>, Box<Self>>),
    List(Vec<Self>),
    Map(Vec<(STRING, Self)>),
    Function(Box<(Vec<(IDENTIFIER,)>, Vec<AST>)>),
    Jcmd(()),
    Jhandle(()),
    Jexit(()),
    Fhandle(()),
}

impl<AST, STRING, IDENTIFIER> Value<AST, STRING, IDENTIFIER> {
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

impl<AST> IOperation<AST> {
    pub fn kind(&self) -> IOperationKind {
        match self {
            IOperation::Add { lhs: _, rhs: _ } => IOperationKind::Add,
            IOperation::Subtract { lhs: _, rhs: _ } => IOperationKind::Subtract,
            IOperation::Multiply { lhs: _, rhs: _ } => IOperationKind::Multiply,
            IOperation::Divide { lhs: _, rhs: _ } => IOperationKind::Divide,
            IOperation::LessThan { lhs: _, rhs: _ } => IOperationKind::LessThan,
            IOperation::GreaterThan { lhs: _, rhs: _ } => IOperationKind::GreaterThan,
            IOperation::Equal { lhs: _, rhs: _ } => IOperationKind::Equal,
            IOperation::Index { lhs: _, rhs: _ } => IOperationKind::Index,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum IOperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,
    Equal,
    Index,
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

impl<AST> POperation<AST> {
    pub fn kind(&self) -> POperationKind {
        match self {
            POperation::Negate { rhs: _ } => POperationKind::Negate,
            POperation::Some { rhs: _ } => POperationKind::Some,
            POperation::Ok { rhs: _ } => POperationKind::Ok,
            POperation::Err { rhs: _ } => POperationKind::Err,
            POperation::ChangeDirectory { rhs: _ } => POperationKind::ChangeDirectory,
            POperation::Run { rhs: _ } => POperationKind::Run,
            POperation::Spawn { rhs: _ } => POperationKind::Spawn,
            POperation::Process { rhs: _ } => POperationKind::Process,
            POperation::Wait { rhs: _ } => POperationKind::Wait,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum POperationKind {
    Negate,
    Some,
    Ok,
    Err,
    ChangeDirectory,
    Run,
    Spawn,
    Process,
    Wait,
}

#[derive(Debug)]
pub enum SpecialOperation<AST, STRING, IDENTIFIER, VALUEKIND> {
    List(Vec<AST>),
    Map(Vec<(STRING, AST)>),
    Drop {
        rhs_iden: IDENTIFIER,
    },
    Let {
        identifier: IDENTIFIER,
        r#type: Option<VALUEKIND>,
        expression: Option<Box<AST>>,
    },
    Assign(()),
    Call {
        lhs: Box<AST>,
        rhs_args: Vec<AST>,
    },
    Shell(STRING),
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

impl<AST, STRING, IDENTIFIER, VALUEKIND> SpecialOperation<AST, STRING, IDENTIFIER, VALUEKIND> {
    pub fn kind(&self) -> SpecialOperationKind {
        match self {
            SpecialOperation::List(_) => SpecialOperationKind::List,
            SpecialOperation::Map(_) => SpecialOperationKind::Map,
            SpecialOperation::Drop { rhs_iden: _ } => SpecialOperationKind::Drop,
            SpecialOperation::Let {
                identifier: _,
                r#type: _,
                expression: _,
            } => SpecialOperationKind::Let,
            SpecialOperation::Assign(()) => SpecialOperationKind::Assign,
            SpecialOperation::Call {
                lhs: _,
                rhs_args: _,
            } => SpecialOperationKind::Call,
            SpecialOperation::Shell(_) => SpecialOperationKind::Shell,
            SpecialOperation::Loop { body: _ } => SpecialOperationKind::Loop,
            SpecialOperation::While {
                condition: _,
                body: _,
            } => SpecialOperationKind::While,
            SpecialOperation::For {
                iterator: _,
                body: _,
            } => SpecialOperationKind::For,
            SpecialOperation::If(_) => SpecialOperationKind::If,
            SpecialOperation::IfElse(_) => SpecialOperationKind::IfElse,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SpecialOperationKind {
    List,
    Map,
    Drop,
    Let,
    Assign,
    Call,
    Shell,
    Loop,
    While,
    For,
    If,
    IfElse,
}
