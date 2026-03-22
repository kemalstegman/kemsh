// let var = [expr];
// let var;
// let var: [type];
// let var: [type] = [expr];
// var = [expr];
// echo [expr];
// cd [expr: string];
// if [expr: boolean] {}
// loop {}
// while [expr: boolean] {}
// ??for??
// ??return [?expr];??
// ??break [?expr];??
// ??fn??

#[derive(Debug)]
pub enum Expression {
    Variable(VariableName),
    Value(VariableValue),
    Operation(Box<Operation>),
    // Instruction,
    // Block,
}

#[derive(Debug)]
pub enum Operation {
    Add { lhs: Expression, rhs: Expression },
    Subtract { lhs: Expression, rhs: Expression },
    Multiply { lhs: Expression, rhs: Expression },
    Divide { lhs: Expression, rhs: Expression },
}

pub type VariableName = String;

#[derive(Debug, Clone)]
pub enum VariableValue {
    LiteralString(String),
    LiteralInteger(i64),
    // LiteralFloating(f64),
    LiteralBoolean(bool),
}

impl VariableValue {
    pub fn kind(&self) -> VariableKind {
        match self {
            VariableValue::LiteralString(_) => VariableKind::LiteralString,
            VariableValue::LiteralInteger(_) => VariableKind::LiteralInteger,
            // VariableValue::LiteralFloating(_) => VariableKind::LiteralFloating,
            VariableValue::LiteralBoolean(_) => VariableKind::LiteralBoolean,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VariableKind {
    LiteralString,
    LiteralInteger,
    LiteralFloating,
    LiteralBoolean,
}

#[derive(Debug)]
pub enum Instruction {
    Let(LetInstruction),
    Set(SetInstruction),
    ChangeDirectory(ChangeDirectoryInstruction),
    Echo(EchoInstruction),
    Exit(ExitInstruction),
}

#[derive(Debug)]
pub struct LetInstruction {
    pub variable_name: VariableName,
    pub variable_kind: Option<VariableKind>,
    pub expression: Option<Expression>,
}
#[derive(Debug)]
pub struct SetInstruction {
    pub variable_name: VariableName,
    pub expression: Expression,
}
#[derive(Debug)]
pub struct ChangeDirectoryInstruction {
    pub expression: Expression,
}
#[derive(Debug)]
pub struct EchoInstruction {
    pub expressions: Vec<Expression>,
}
#[derive(Debug)]
pub struct ExitInstruction {
    pub expression: Expression,
}
