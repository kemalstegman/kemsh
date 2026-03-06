pub enum UnitInstruction {
    Let(LetCommand),
    Assign(AssignCommand),
    For,
    While,
    Loop,
    Return,
    Break,
    Run,
    Spawn,
    CD,
}

pub struct LetCommand {
    pub variable: String,
    // variable_type: Option<_>,
    pub value: Expression,
}

pub struct AssignCommand {
    pub variable: String,
    pub value: Expression,
}

pub enum Expression {
    Literal(Literal),
    Variable(String),
    Operation(Box<Operation>),
    Block(BraceBlock),
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

pub enum Operation {
    Add { lhs: Expression, rhs: Expression },
    Subtract { lhs: Expression, rhs: Expression },
    Multiply { lhs: Expression, rhs: Expression },
    Divide { lhs: Expression, rhs: Expression },
}

pub struct BraceBlock {
    pub label: Option<String>,
    pub vec: Vec<UnitInstruction>,
}
