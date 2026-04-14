use crate::executor::concrete::ConcreteKind;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    CompoundLiteral(CompoundLiteral),
    Identifier(Identifier),
    Operation(Box<Operation>),
    BraceBlock(BraceBlock),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    RawString(String),
}

#[derive(Debug, Clone)]
pub enum CompoundLiteral {
    // EscapedString,
    List(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct BraceBlock {
    pub expressions: Vec<Expression>,
    pub evaluate_to_tail_expression: bool,
}

#[derive(Debug, Clone)]
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
    /// run a job
    Run(Expression),
    /// spawn a job
    Spawn(Expression),
    /// build a job
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
    /// echo out an expression without a newline
    Echon(Expression),
    /// exit with a status
    Exit(Expression),
}

#[derive(Debug, Clone)]
pub struct DeclareLValue {
    pub identifier: Identifier,
    pub type_annotation: Option<ConcreteKind>,
}

#[derive(Debug, Clone)]
pub struct MutableLValue {
    pub identifier: Identifier,
    pub type_annotation: Option<ConcreteKind>,
    pub indices: Vec<Expression>,
}
