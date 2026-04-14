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
    // JCmd,
    // JHandle,
    // JExit,
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
    // JCmd
    // JHandle
    // JExit
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
