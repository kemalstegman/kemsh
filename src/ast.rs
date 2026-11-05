#[derive(Debug)]
pub enum AST<'a> {
    Literal(Literal<'a>),
    Identifier(&'a str),
    Operation(Operation<'a>),
}

#[derive(Debug)]
pub enum Literal<'a> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(&'a str),
}

#[derive(Debug)]
pub enum Operation<'a> {
    Add(Box<(AST<'a>, AST<'a>)>),
    Subtract(Box<(AST<'a>, AST<'a>)>),
    Multiply(Box<(AST<'a>, AST<'a>)>),
    Divide(Box<(AST<'a>, AST<'a>)>),
    Negate(Box<AST<'a>>),
}
