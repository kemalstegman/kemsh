use std::{collections::HashMap, rc::Rc};

use crate::ast::{AST, OwnedText, OwnedValues, TextOwnership, Value, ValueKind, ValueOwnership};

pub struct BindingTable(Vec<Scope>);

impl BindingTable {
    fn new() -> Self {
        Self(vec![Scope::new()])
    }
}

pub struct ScopeRegistry(Vec<Scope>);

impl ScopeRegistry {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    fn get(&self, identifier: &str) -> Option<&Variable> {
        self.0
            .iter()
            .rev()
            .map(|s| s.get(identifier))
            .filter(|o| o.is_some())
            .next()
            .flatten()
    }
    fn remove(&mut self, identifier: &str) -> Option<(String, Variable)> {
        self.0.last_mut().map(|s| s.remove(identifier)).flatten()
    }
    fn declare(&mut self, identifier: String, variable: Variable) -> Result<(), ()> {
        self.0
            .last_mut()
            .map(|s| s.declare(identifier, variable))
            .unwrap()
    }
    fn assign(
        &mut self,
        identifier: &str,
        value: Rc<Value<IdentifierOwned, ValueShared>>,
    ) -> Result<(), ()> {
        let value = &value;
        self.0
            .iter_mut()
            .rev()
            .map(|s| s.assign(identifier, Rc::clone(value)))
            .filter(|r| r.is_ok())
            .next()
            .unwrap()
    }
}

pub struct Scope(HashMap<String, Variable>);

impl Scope {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn get(&self, identifier: &str) -> Option<&Variable> {
        self.0.get(identifier)
    }
    fn remove(&mut self, identifier: &str) -> Option<(String, Variable)> {
        self.0.remove_entry(identifier)
    }
    fn declare(&mut self, identifier: String, variable: Variable) -> Result<(), ()> {
        match self.0.get(&identifier) {
            Some(_) => todo!(),
            None => {
                self.0.insert(identifier, variable);
                Ok(())
            }
        }
    }
    fn assign(&mut self, identifier: &str, value: Rc<Value<ASTOwned>>) -> Result<(), ()> {
        match self.0.get_mut(identifier) {
            None => Err(()),
            Some(v) => match v {
                Variable::Declared => {
                    *v = Variable::Value(value);
                    Ok(())
                }
                Variable::TypeBound(t) if *t == value.kind() => {
                    *v = Variable::Value(value);
                    Ok(())
                }
                Variable::TypeBound(_) => todo!(),
                Variable::Value(stored_value) if stored_value.kind() == value.kind() => {
                    *v = Variable::Value(value);
                    Ok(())
                }
                Variable::Value(_) => todo!(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Variable {
    Declared,
    TypeBound(ValueKind),
    Value(Rc<Value<'static, OwnedText, OwnedValues>>),
}

#[derive(Debug)]
pub enum FatalEvaluationError {}

pub fn evaluate<'a, TO: TextOwnership, VO: ValueOwnership>(
    ast: AST<'a, TO, VO>,
    scope_registry: &mut ScopeRegistry,
) -> Result<Rc<Value<'a>>, FatalEvaluationError> {
    match ast {
        AST::Value(v) => Ok(Rc::new(v)),
        AST::Identifier(identifier) => match scope_registry.get(identifier) {
            Some(Variable::Value(v)) => todo!(),
            Some(Variable::TypeBound(_) | Variable::Declared) => todo!(),
            None => todo!(),
        },
        AST::Control(Control::Loop(_)) => todo!(),
        AST::Control(Control::While(_, _)) => todo!(),
        AST::Control(Control::For(_, _)) => todo!(),
        AST::Control(Control::IfElse(_)) => todo!(),
        AST::Control(Control::If(_)) => todo!(),
        AST::BOperation(boperation) => {
            Ok(Rc::new(evaluate_boperation(*boperation, scope_registry)?))
        }
        AST::VOperation(VOperation::Call(ast, _args)) => match evaluate(*ast, scope_registry)? {
            _ => todo!(),
        },
        AST::VOperation(VOperation::List(elements)) => Ok(Rc::new(Value::List(
            elements
                .into_iter()
                .map(|ast| evaluate(ast, scope_registry))
                .collect::<Result<Vec<Rc<Value<'a>>>, FatalEvaluationError>>()?,
        ))),
        AST::VOperation(VOperation::Map(pairs)) => Ok(Rc::new(Value::Map(
            pairs
                .into_iter()
                .map(|(key, value_ast)| {
                    evaluate(value_ast, scope_registry).map(|value| (key, value))
                })
                .collect::<Result<Vec<(&'a str, Value<'a>)>, FatalEvaluationError>>()?,
        ))),
    }
}

pub fn evaluate_boperation<'a>(
    boperation: BOperation<'a>,
    scope_registry: &mut ScopeRegistry,
) -> Result<Value<'a>, FatalEvaluationError> {
    match boperation {
        BOperation::Add(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::I64(lhs + rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::U64(lhs + rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::F64(lhs + rhs)),
            _ => todo!(),
        },
        BOperation::Subtract(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::I64(lhs - rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::U64(lhs - rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::F64(lhs - rhs)),
            _ => todo!(),
        },
        BOperation::Multiply(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::I64(lhs * rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::U64(lhs * rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::F64(lhs * rhs)),
            _ => todo!(),
        },
        BOperation::Divide(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::I64(lhs / rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::U64(lhs / rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::F64(lhs / rhs)),
            _ => todo!(),
        },
        BOperation::Negate(rhs) => match evaluate(rhs, scope_registry)? {
            Value::BOOL(boolean) => Ok(Value::BOOL(!boolean)),
            _ => todo!(),
        },
        BOperation::Some(rhs) => Ok(Value::Option(Some(Box::new(evaluate(
            rhs,
            scope_registry,
        )?)))),
        BOperation::Ok(rhs) => Ok(Value::Result(Ok(Box::new(evaluate(rhs, scope_registry)?)))),
        BOperation::Err(rhs) => Ok(Value::Result(Err(Box::new(evaluate(rhs, scope_registry)?)))),
        BOperation::ChangeDirectory(_) => todo!(),
        BOperation::Run(_) => todo!(),
        BOperation::Spawn(_) => todo!(),
        BOperation::Process(_) => todo!(),
        BOperation::Wait(_) => todo!(),
        BOperation::Drop { identifier: _ } => todo!(),
        BOperation::Let {
            identifier,
            r#type,
            expression,
        } => match (
            r#type,
            expression
                .map(|ast| evaluate(ast, scope_registry))
                .transpose()?,
        ) {
            (None, None) => todo!(),
            (Some(r#type), None) => todo!(),
            (None, Some(value)) => todo!(),
            (Some(r#type), Some(value)) => todo!(),
        },
        BOperation::Assign(_) => todo!(),
        BOperation::LessThan(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::BOOL(lhs < rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::BOOL(lhs < rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::BOOL(lhs < rhs)),
            _ => todo!(),
        },
        BOperation::GreaterThan(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::BOOL(lhs > rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::BOOL(lhs > rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::BOOL(lhs > rhs)),
            _ => todo!(),
        },
        BOperation::Equal(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            (Value::I64(lhs), Value::I64(rhs)) => Ok(Value::BOOL(lhs == rhs)),
            (Value::U64(lhs), Value::U64(rhs)) => Ok(Value::BOOL(lhs == rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(Value::BOOL(lhs == rhs)),
            (Value::BOOL(lhs), Value::BOOL(rhs)) => Ok(Value::BOOL(lhs == rhs)),
            (Value::STRING(lhs), Value::STRING(rhs)) => Ok(Value::BOOL(lhs == rhs)),
            (Value::UNIT, Value::UNIT) => Ok(Value::BOOL(true)),
            _ => todo!(),
        },
        BOperation::Index(lhs, rhs) => match (
            evaluate(lhs, scope_registry)?,
            evaluate(rhs, scope_registry)?,
        ) {
            _ => todo!(),
        },
        BOperation::Shell(_) => todo!(),
    }
}

// pub enum Value<'a, TO: TextOwnership> {
//     I64(i64),
//     U64(u64),
//     F64(f64),
//     BOOL(bool),
//     STRING(TO::Text<'a>),
//     UNIT,
//     Option(Option<Box<Self>>),
//     Result(Result<Box<Self>, Box<Self>>),
//     List(Vec<Self>),
//     Map(Vec<(TO::Text<'a>, Self)>),
//     Function(Box<(Vec<(TO::Text<'a>,)>, Vec<AST<'a, TO>>)>),
//     Jcmd(()),
//     Jhandle(()),
//     Jexit(()),
//     Fhandle(()),
// }
//
// impl<'a, TO: TextOwnership> Value<'a, TO> {
//     pub fn kind(&self) -> ValueKind {
//         match self {
//             Value::I64(_) => ValueKind::I64,
//             Value::U64(_) => ValueKind::U64,
//             Value::F64(_) => ValueKind::F64,
//             Value::BOOL(_) => ValueKind::BOOL,
//             Value::STRING(_) => ValueKind::STRING,
//             Value::UNIT => ValueKind::UNIT,
//             Value::Option(_) => ValueKind::Option,
//             Value::Result(_) => ValueKind::Result,
//             Value::List(_) => ValueKind::List,
//             Value::Map(_) => ValueKind::Map,
//             Value::Function(_) => ValueKind::Function,
//             Value::Jcmd(_) => ValueKind::Jcmd,
//             Value::Jhandle(_) => ValueKind::Jhandle,
//             Value::Jexit(_) => ValueKind::Jexit,
//             Value::Fhandle(_) => ValueKind::Fhandle,
//         }
//     }
// }
