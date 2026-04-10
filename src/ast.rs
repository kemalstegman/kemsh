#[derive(Debug)]
pub enum Expression {
    Concrete(Concrete),
    Identifier(Identifier),
    Operation(Box<Operation>),
    BraceBlock(BraceBlock),
}

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct BraceBlock(pub Vec<Expression>, pub Option<Box<Expression>>);

#[derive(Debug, Clone)]
pub enum Concrete {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    List(Vec<Concrete>),
    Map(std::collections::HashMap<String, Concrete>),
    Option(Option<Box<Concrete>>),
    Result(Result<Box<Concrete>, Box<Concrete>>),
}

impl Concrete {
    pub fn kind(&self) -> ConcreteKind {
        match self {
            Concrete::Integer(_) => ConcreteKind::Integer,
            Concrete::Float(_) => ConcreteKind::Float,
            Concrete::Boolean(_) => ConcreteKind::Boolean,
            Concrete::String(_) => ConcreteKind::String,
            Concrete::List(_) => ConcreteKind::List,
            Concrete::Map(_) => ConcreteKind::Map,
            Concrete::Option(_) => ConcreteKind::Option,
            Concrete::Result(_) => ConcreteKind::Result,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConcreteKind {
    Integer,
    Float,
    Boolean,
    String,
    List,
    Map,
    Option,
    Result,
    // PCmd
    // PHandle
    // PExit
    // Function
}

#[derive(Clone)]
pub enum VoidConcrete {
    Void,
    NonVoid(Concrete),
}

impl VoidConcrete {
    pub fn kind(&self) -> VoidConcreteKind {
        match self {
            VoidConcrete::Void => VoidConcreteKind::Void,
            VoidConcrete::NonVoid(concrete) => VoidConcreteKind::NonVoid(concrete.kind()),
        }
    }
}

impl std::fmt::Debug for VoidConcrete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoidConcrete::Void => write!(f, "Void"),
            VoidConcrete::NonVoid(concrete) => write!(f, "{concrete:?}"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VoidConcreteKind {
    Void,
    NonVoid(ConcreteKind),
}

impl std::fmt::Debug for VoidConcreteKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoidConcreteKind::Void => write!(f, "Void"),
            VoidConcreteKind::NonVoid(concrete_kind) => write!(f, "{concrete_kind:?}"),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    // infix
    /// add two integers or floats, or concatenate two strings
    AddConcat { lhs: Expression, rhs: Expression },
    /// subtract two integers or floats
    Subtract { lhs: Expression, rhs: Expression },
    /// multiply two integers or floats
    Multiply { lhs: Expression, rhs: Expression },
    /// divide two integers or floats
    Divide { lhs: Expression, rhs: Expression },
    /// modulo two integers or floats
    Modulo { lhs: Expression, rhs: Expression },
    /// pipe two processes
    Pipe { lhs: Expression, rhs: Expression },
    /// compare equality
    ComparisonEqual { lhs: Expression, rhs: Expression },
    /// assign an lvalue to an rvalue
    Assign { lhs: MutableLValue, rhs: Expression },
    /// assign an lvalue to void, deleting it.
    AssignVoid { lhs: Identifier },
    /// index into a list or map
    Index { lhs: Expression, rhs: Expression },

    // unary prefix
    /// negate an integer or float
    Negate(Expression),
    /// block on a process
    Block(Expression),
    /// spawn a process
    Spawn(Expression),
    /// build a command
    Command(Expression),
    /// change the current working directory
    ChangeDirectory(Expression),
    /// declare
    Let {
        lhs: DeclareLValue,
        rhs: Option<Expression>,
    },
    /// echo out an expression
    Echo(Expression),
    /// exit with a status
    Exit(Expression),
}

#[derive(Debug)]
pub struct DeclareLValue {
    pub identifier: Identifier,
    pub type_annotation: Option<ConcreteKind>,
}

#[derive(Debug)]
pub struct MutableLValue {
    pub identifier: Identifier,
    pub type_annotation: Option<ConcreteKind>,
    pub indices: Vec<Expression>,
}

pub trait Environment {
    type Error;
    fn declare(
        &mut self,
        identifier: Identifier,
        value: Option<Concrete>,
    ) -> Result<(), Self::Error>;
    fn assign(&mut self, identifier: &Identifier, value: Concrete) -> Result<(), Self::Error>;
    fn get(&self, identifier: &Identifier) -> Result<Concrete, Self::Error>;
}
