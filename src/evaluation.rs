use std::{
    ops::{Neg, Not},
    rc::Rc,
};

use crate::ast::{AST, IOperation, POperation, SpecialOperation, TextOwnership, ValueKind};

#[derive(Debug, Clone)]
pub enum Value {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(String),
    UNIT,
    Option(Option<Rc<Value>>),
    Result(Result<Rc<Value>, Rc<Value>>),
    List(Vec<Rc<Value>>),
    Map(Vec<(String, Rc<Value>)>),
    Function,
    // Shell,
    // Jcmd,
    // Jhandle,
    // Jexit,
    // Fhandle,
}

impl Value {
    fn as_value_ref<'a>(&'a self) -> ValueRef<'a> {
        match self {
            Value::I64(num) => ValueRef::I64(*num),
            Value::U64(num) => ValueRef::U64(*num),
            Value::F64(num) => ValueRef::F64(*num),
            Value::BOOL(boolean) => ValueRef::BOOL(*boolean),
            Value::STRING(string) => ValueRef::STRING(string.as_str()),
            Value::UNIT => ValueRef::UNIT,
            Value::Option(option) => ValueRef::Option(option.as_ref().map(|o| o.as_ref())),
            Value::Result(result) => {
                ValueRef::Result(result.as_ref().map(|o| o.as_ref()).map_err(|e| e.as_ref()))
            }
            Value::List(list) => ValueRef::List(list.as_slice()),
            Value::Map(map) => ValueRef::Map(map.as_slice()),
            Value::Function => ValueRef::Function,
        }
    }
}

pub enum ValueRef<'a> {
    I64(i64),
    U64(u64),
    F64(f64),
    BOOL(bool),
    STRING(&'a str),
    UNIT,
    Option(Option<&'a Value>),
    Result(Result<&'a Value, &'a Value>),
    List(&'a [Rc<Value>]),
    Map(&'a [(String, Rc<Value>)]),
    Function,
    // Shell,
    // Jcmd,
    // Jhandle,
    // Jexit,
    // Fhandle,
}

#[derive(Debug)]
pub enum EvalValue<'a> {
    Owned(Value),
    Shared(Rc<Value>),

    BorrowedString(&'a str),
}

impl<'a> EvalValue<'a> {
    fn convert_own_life_shared(self) -> Rc<Value> {
        match self {
            Self::Owned(v) => Rc::new(v),
            Self::Shared(v) => v,
            Self::BorrowedString(s) => Rc::new(Value::STRING(s.to_owned())),
        }
    }
    fn as_mut_owned_or_ref(&'a mut self) -> Result<&'a mut Value, ValueRef<'a>> {
        match self {
            Self::Owned(v) => Ok(v),
            Self::Shared(v) => Err(v.as_value_ref()),
            Self::BorrowedString(s) => Err(ValueRef::STRING(s)),
        }
    }
    fn as_value_ref(&self) -> ValueRef<'_> {
        match self {
            Self::Owned(v) => v.as_value_ref(),
            Self::Shared(v) => v.as_value_ref(),
            Self::BorrowedString(s) => ValueRef::STRING(s),
        }
    }
}

pub fn egv(_: &str) -> Rc<Value> {
    Rc::new(Value::UNIT)
}

pub fn evaluate<'a, TO: TextOwnership>(
    ast: &'a AST<'_, TO>,
    env_get_variable: fn(&str) -> Rc<Value>,
) -> Result<EvalValue<'a>, ()> {
    match ast {
        AST::I64(num) => Ok(EvalValue::Owned(Value::I64(*num))),
        AST::U64(num) => Ok(EvalValue::Owned(Value::U64(*num))),
        AST::F64(num) => Ok(EvalValue::Owned(Value::F64(*num))),
        AST::BOOL(boolean) => Ok(EvalValue::Owned(Value::BOOL(*boolean))),
        AST::STRING(string) => Ok(EvalValue::BorrowedString(TO::as_str(string))),
        AST::UNIT => Ok(EvalValue::Owned(Value::UNIT)),
        AST::NONE => Ok(EvalValue::Owned(Value::Option(None))),
        AST::Identifier(identifier) => {
            Ok(EvalValue::Shared(env_get_variable(TO::as_str(identifier))))
        }
        AST::POperation(operation) => match &**operation {
            POperation::Ok { rhs } => Ok(EvalValue::Owned(Value::Result(Ok(evaluate(
                rhs,
                env_get_variable,
            )?
            .convert_own_life_shared())))),
            POperation::Err { rhs } => Ok(EvalValue::Owned(Value::Result(Err(evaluate(
                rhs,
                env_get_variable,
            )?
            .convert_own_life_shared())))),
            POperation::Some { rhs } => Ok(EvalValue::Owned(Value::Option(Some(
                evaluate(rhs, env_get_variable)?.convert_own_life_shared(),
            )))),
            POperation::Negate { rhs } => match evaluate(rhs, env_get_variable)?.as_value_ref() {
                ValueRef::I64(num) => Ok(EvalValue::Owned(Value::I64(num.neg()))),
                ValueRef::F64(num) => Ok(EvalValue::Owned(Value::F64(num.neg()))),
                _ => todo!(),
            },
            POperation::ChangeDirectory { rhs } => todo!(),
            POperation::Run { rhs } => todo!(),
            POperation::Spawn { rhs } => todo!(),
            POperation::Process { rhs } => todo!(),
            POperation::Wait { rhs } => todo!(),
        },
        AST::IOperation(operation) => match &**operation {
            IOperation::Add { lhs, rhs } => {
                let mut lhs = evaluate(lhs, env_get_variable)?;
                let mut rhs = evaluate(rhs, env_get_variable)?;
                let (lhs, rhs) = match (lhs.as_mut_owned_or_ref(), rhs.as_mut_owned_or_ref()) {
                    (Ok(Value::STRING(lhs)), Ok(Value::STRING(rhs))) => {
                        lhs.push_str(rhs);
                        return Ok(EvalValue::Owned(Value::STRING(std::mem::replace(
                            lhs,
                            String::new(),
                        ))));
                    }
                    (Ok(Value::STRING(owned)), Err(ValueRef::STRING(slice)))
                    | (Err(ValueRef::STRING(slice)), Ok(Value::STRING(owned))) => {
                        owned.push_str(slice);
                        return Ok(EvalValue::Owned(Value::STRING(std::mem::replace(
                            owned,
                            String::new(),
                        ))));
                    }
                    (Ok(lhs), Ok(rhs)) => (lhs.as_value_ref(), rhs.as_value_ref()),
                    (Ok(lhs), Err(rhs)) => (lhs.as_value_ref(), rhs),
                    (Err(lhs), Ok(rhs)) => (lhs, rhs.as_value_ref()),
                    (Err(lhs), Err(rhs)) => (lhs, rhs),
                };
                match (lhs, rhs) {
                    (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                        Ok(EvalValue::Owned(Value::I64(lhs + rhs)))
                    }
                    (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                        Ok(EvalValue::Owned(Value::U64(lhs + rhs)))
                    }
                    (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                        Ok(EvalValue::Owned(Value::F64(lhs + rhs)))
                    }
                    (ValueRef::STRING(lhs), ValueRef::STRING(rhs)) => {
                        Ok(EvalValue::Owned(Value::STRING(lhs.to_owned() + rhs)))
                    }
                    _ => todo!(),
                }
            }
            IOperation::Subtract { lhs, rhs } => match (
                evaluate(lhs, env_get_variable)?.as_value_ref(),
                evaluate(rhs, env_get_variable)?.as_value_ref(),
            ) {
                (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                    Ok(EvalValue::Owned(Value::I64(lhs - rhs)))
                }
                (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                    Ok(EvalValue::Owned(Value::U64(lhs - rhs)))
                }
                (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                    Ok(EvalValue::Owned(Value::F64(lhs - rhs)))
                }
                _ => todo!(),
            },
            IOperation::Multiply { lhs, rhs } => match (
                evaluate(lhs, env_get_variable)?.as_value_ref(),
                evaluate(rhs, env_get_variable)?.as_value_ref(),
            ) {
                (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                    Ok(EvalValue::Owned(Value::I64(lhs * rhs)))
                }
                (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                    Ok(EvalValue::Owned(Value::U64(lhs * rhs)))
                }
                (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                    Ok(EvalValue::Owned(Value::F64(lhs * rhs)))
                }
                _ => todo!(),
            },
            IOperation::Divide { lhs, rhs } => match (
                evaluate(lhs, env_get_variable)?.as_value_ref(),
                evaluate(rhs, env_get_variable)?.as_value_ref(),
            ) {
                (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                    Ok(EvalValue::Owned(Value::I64(lhs / rhs)))
                }
                (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                    Ok(EvalValue::Owned(Value::U64(lhs / rhs)))
                }
                (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                    Ok(EvalValue::Owned(Value::F64(lhs / rhs)))
                }
                _ => todo!(),
            },
            IOperation::LessThan { lhs, rhs } => match (
                evaluate(lhs, env_get_variable)?.as_value_ref(),
                evaluate(rhs, env_get_variable)?.as_value_ref(),
            ) {
                (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs < rhs)))
                }
                (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs < rhs)))
                }
                (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs < rhs)))
                }
                _ => todo!(),
            },
            IOperation::GreaterThan { lhs, rhs } => match (
                evaluate(lhs, env_get_variable)?.as_value_ref(),
                evaluate(rhs, env_get_variable)?.as_value_ref(),
            ) {
                (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs > rhs)))
                }
                (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs > rhs)))
                }
                (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs > rhs)))
                }
                _ => todo!(),
            },
            IOperation::Equal { lhs, rhs } => match (
                evaluate(lhs, env_get_variable)?.as_value_ref(),
                evaluate(rhs, env_get_variable)?.as_value_ref(),
            ) {
                (ValueRef::I64(lhs), ValueRef::I64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs == rhs)))
                }
                (ValueRef::U64(lhs), ValueRef::U64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs == rhs)))
                }
                (ValueRef::F64(lhs), ValueRef::F64(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs == rhs)))
                }
                (ValueRef::STRING(lhs), ValueRef::STRING(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs == rhs)))
                }
                (ValueRef::BOOL(lhs), ValueRef::BOOL(rhs)) => {
                    Ok(EvalValue::Owned(Value::BOOL(lhs == rhs)))
                }
                _ => todo!(),
            },
            IOperation::Index { lhs, rhs } => todo!(),
        },
        AST::SpecialOperation(operation) => match &*operation {
            SpecialOperation::Assign(()) => todo!(),
            SpecialOperation::List(list) => todo!(),
            SpecialOperation::Map(map) => todo!(),
            SpecialOperation::Drop { rhs_iden } => todo!(),
            SpecialOperation::Let {
                identifier,
                r#type,
                expression,
            } => todo!(),
            SpecialOperation::Call { lhs, rhs_args } => todo!(),
            SpecialOperation::For { iterator, body } => todo!(),
            SpecialOperation::If(r#if) => todo!(),
            SpecialOperation::IfElse(if_else) => todo!(),
            SpecialOperation::Loop { body } => todo!(),
            SpecialOperation::Shell(shell) => todo!(),
            SpecialOperation::While { condition, body } => todo!(),
        },
    }
}
