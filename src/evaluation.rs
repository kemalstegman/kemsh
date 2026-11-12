use std::{
    collections::HashMap,
    ops::{Neg, Not},
    rc::Rc,
};

use crate::ast::{AST, IOperation, POperation, SpecialOperation, TextOwnership, ValueKind};

pub struct Environment {
    bindings: Bindings,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: Bindings::new_with_default_globals(),
        }
    }
}

pub struct Bindings(Vec<BindingScope>);

impl Bindings {
    pub fn new_with_default_globals() -> Self {
        Self(vec![BindingScope::new_with_default_globals()])
    }
    pub fn declare(&mut self, identifier: String, variable: Variable) -> Result<(), BindingError> {
        self.0
            .last_mut()
            .ok_or(BindingError::NoBindingScopes)?
            .declare(identifier, variable)
    }
    pub fn drop(&mut self, identifier: &str) -> Result<Variable, BindingError> {
        self.0
            .last_mut()
            .ok_or(BindingError::NoBindingScopes)?
            .drop(identifier)
    }
    pub fn assign(
        &mut self,
        identifier: &str,
        value: Rc<Value>,
    ) -> Result<Option<Rc<Value>>, BindingError> {
        if self.0.len() == 0 {
            Err(BindingError::NoBindingScopes)
        } else {
            self.0
                .iter_mut()
                .rev()
                .filter(|scope| scope.is_declared(identifier))
                .next()
                .ok_or(BindingError::AssignNotDeclared)?
                .assign(identifier, value)
        }
    }
    pub fn get(&self, identifier: &str) -> Result<&Rc<Value>, BindingError> {
        if self.0.len() == 0 {
            Err(BindingError::NoBindingScopes)
        } else {
            self.0
                .iter()
                .rev()
                .map(|scope| scope.get(identifier))
                .find_map(Result::ok)
                .ok_or(BindingError::GetNotDeclared)
        }
    }
}

pub struct BindingScope(HashMap<String, Variable>);

impl BindingScope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn new_with_default_globals() -> Self {
        let mut binding_scope = BindingScope::new();
        binding_scope
    }
    pub fn declare(&mut self, identifier: String, variable: Variable) -> Result<(), BindingError> {
        use std::collections::hash_map::Entry;
        match self.0.entry(identifier) {
            Entry::Vacant(entry) => {
                entry.insert_entry(variable);
                Ok(())
            }
            Entry::Occupied(_) => Err(BindingError::DeclareAlreadyDeclared),
        }
    }
    pub fn drop(&mut self, identifier: &str) -> Result<Variable, BindingError> {
        self.0
            .remove(identifier)
            .ok_or(BindingError::DropNotDeclared)
    }
    pub fn is_declared(&self, identifier: &str) -> bool {
        self.0.contains_key(identifier)
    }
    pub fn assign(
        &mut self,
        identifier: &str,
        value: Rc<Value>,
    ) -> Result<Option<Rc<Value>>, BindingError> {
        match self.0.get_mut(identifier) {
            None => Err(BindingError::AssignNotDeclared),
            Some(variable) => variable
                .assign(value)
                .map_err(|()| BindingError::AssignVariableDifferentType),
        }
    }
    pub fn get(&self, identifier: &str) -> Result<&Rc<Value>, BindingError> {
        match self.0.get(identifier).ok_or(BindingError::GetNotDeclared)? {
            Variable::Declared => Err(BindingError::GetVariableDeclared),
            Variable::Typed(_) => Err(BindingError::GetVariableTyped),
            Variable::Valued(value) => Ok(&value),
        }
    }
}

pub enum Variable {
    Declared,
    Typed(ValueKind),
    Valued(Rc<Value>),
}

#[derive(Debug)]
pub enum BindingError {
    DeclareAlreadyDeclared,
    DropNotDeclared,
    AssignNotDeclared,
    AssignVariableDifferentType,
    LetVariableDifferentType,
    GetNotDeclared,
    GetVariableDeclared,
    GetVariableTyped,
    NoBindingScopes,
}

impl Variable {
    fn assign(&mut self, assign_value: Rc<Value>) -> Result<Option<Rc<Value>>, ()> {
        match self {
            Variable::Declared => (),
            Variable::Typed(r#type) if *r#type == assign_value.r#type() => (),
            Variable::Valued(value) if value.r#type() == assign_value.r#type() => {
                return Ok(Some(std::mem::replace(value, assign_value)));
            }
            Variable::Typed(_) | Variable::Valued(_) => return Err(()),
        }
        *self = Variable::Valued(assign_value);
        Ok(None)
    }
}

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
    fn r#type(&self) -> ValueKind {
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
            Value::Function => ValueKind::Function,
        }
    }
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

#[derive(Debug)]
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

impl<'a> ValueRef<'a> {
    fn r#type(&self) -> ValueKind {
        match self {
            ValueRef::I64(_) => ValueKind::I64,
            ValueRef::U64(_) => ValueKind::U64,
            ValueRef::F64(_) => ValueKind::F64,
            ValueRef::BOOL(_) => ValueKind::BOOL,
            ValueRef::STRING(_) => ValueKind::STRING,
            ValueRef::UNIT => ValueKind::UNIT,
            ValueRef::Option(_) => ValueKind::Option,
            ValueRef::Result(_) => ValueKind::Result,
            ValueRef::List(_) => ValueKind::List,
            ValueRef::Map(_) => ValueKind::Map,
            ValueRef::Function => ValueKind::Function,
        }
    }
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

#[derive(Debug)]
pub enum EvaluationError {
    BindingError(BindingError),
    G(String),
}

pub fn evaluate<'a, TO: TextOwnership>(
    ast: &'a AST<'_, TO>,
    environment: &mut Environment,
) -> Result<EvalValue<'a>, EvaluationError> {
    match ast {
        AST::I64(num) => Ok(EvalValue::Owned(Value::I64(*num))),
        AST::U64(num) => Ok(EvalValue::Owned(Value::U64(*num))),
        AST::F64(num) => Ok(EvalValue::Owned(Value::F64(*num))),
        AST::BOOL(boolean) => Ok(EvalValue::Owned(Value::BOOL(*boolean))),
        AST::STRING(string) => Ok(EvalValue::BorrowedString(TO::as_str(string))),
        AST::UNIT => Ok(EvalValue::Owned(Value::UNIT)),
        AST::NONE => Ok(EvalValue::Owned(Value::Option(None))),
        AST::Identifier(identifier) => Ok(EvalValue::Shared(Rc::clone(
            environment
                .bindings
                .get(TO::as_str(identifier))
                .map_err(|e| EvaluationError::BindingError(e))?,
        ))),
        AST::POperation(operation) => match &**operation {
            POperation::Ok { rhs } => Ok(EvalValue::Owned(Value::Result(Ok(evaluate(
                rhs,
                environment,
            )?
            .convert_own_life_shared())))),
            POperation::Err { rhs } => Ok(EvalValue::Owned(Value::Result(Err(evaluate(
                rhs,
                environment,
            )?
            .convert_own_life_shared())))),
            POperation::Some { rhs } => Ok(EvalValue::Owned(Value::Option(Some(
                evaluate(rhs, environment)?.convert_own_life_shared(),
            )))),
            POperation::Not { rhs } => match evaluate(rhs, environment)?.as_value_ref() {
                ValueRef::BOOL(boolean) => Ok(EvalValue::Owned(Value::BOOL(boolean.not()))),
                value => Err(EvaluationError::G(format!(
                    "invalid not operation on value type {:?}: NOT {:?}",
                    value.r#type(),
                    value
                ))),
            },
            POperation::Negate { rhs } => match evaluate(rhs, environment)?.as_value_ref() {
                ValueRef::I64(num) => Ok(EvalValue::Owned(Value::I64(num.neg()))),
                ValueRef::F64(num) => Ok(EvalValue::Owned(Value::F64(num.neg()))),
                value => Err(EvaluationError::G(format!(
                    "invalid negation operation on value type {:?}: NEGATE {:?}",
                    value.r#type(),
                    value
                ))),
            },
            POperation::ChangeDirectory { rhs } => todo!(),
            POperation::Run { rhs } => todo!(),
            POperation::Spawn { rhs } => todo!(),
            POperation::Process { rhs } => todo!(),
            POperation::Wait { rhs } => todo!(),
        },
        AST::IOperation(operation) => match &**operation {
            IOperation::Add { lhs, rhs } => {
                let mut lhs = evaluate(lhs, environment)?;
                let mut rhs = evaluate(rhs, environment)?;
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
                    (lvalue, rvalue) => Err(EvaluationError::G(format!(
                        "invalid addition operation on lhs value type {:?} and rhs value type {:?}: {:?} ADD {:?}",
                        lvalue.r#type(),
                        rvalue.r#type(),
                        lvalue,
                        rvalue
                    ))),
                }
            }
            IOperation::Subtract { lhs, rhs } => match (
                evaluate(lhs, environment)?.as_value_ref(),
                evaluate(rhs, environment)?.as_value_ref(),
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
                (lvalue, rvalue) => Err(EvaluationError::G(format!(
                    "invalid subtraction operation on lhs value type {:?} and rhs value type {:?}: {:?} SUBTRACT {:?}",
                    lvalue.r#type(),
                    rvalue.r#type(),
                    lvalue,
                    rvalue
                ))),
            },
            IOperation::Multiply { lhs, rhs } => match (
                evaluate(lhs, environment)?.as_value_ref(),
                evaluate(rhs, environment)?.as_value_ref(),
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
                (lvalue, rvalue) => Err(EvaluationError::G(format!(
                    "invalid multiplication operation on lhs value type {:?} and rhs value type {:?}: {:?} MULTIPLY {:?}",
                    lvalue.r#type(),
                    rvalue.r#type(),
                    lvalue,
                    rvalue
                ))),
            },
            IOperation::Divide { lhs, rhs } => match (
                evaluate(lhs, environment)?.as_value_ref(),
                evaluate(rhs, environment)?.as_value_ref(),
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
                (lvalue, rvalue) => Err(EvaluationError::G(format!(
                    "invalid division operation on lhs value type {:?} and rhs value type {:?}: {:?} DIVIDE {:?}",
                    lvalue.r#type(),
                    rvalue.r#type(),
                    lvalue,
                    rvalue
                ))),
            },
            IOperation::LessThan { lhs, rhs } => match (
                evaluate(lhs, environment)?.as_value_ref(),
                evaluate(rhs, environment)?.as_value_ref(),
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
                (lvalue, rvalue) => Err(EvaluationError::G(format!(
                    "invalid less than operation on lhs value type {:?} and rhs value type {:?}: {:?} LESS THAN {:?}",
                    lvalue.r#type(),
                    rvalue.r#type(),
                    lvalue,
                    rvalue
                ))),
            },
            IOperation::GreaterThan { lhs, rhs } => match (
                evaluate(lhs, environment)?.as_value_ref(),
                evaluate(rhs, environment)?.as_value_ref(),
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
                (lvalue, rvalue) => Err(EvaluationError::G(format!(
                    "invalid greater than operation on lhs value type {:?} and rhs value type {:?}: {:?} GREATER THAN {:?}",
                    lvalue.r#type(),
                    rvalue.r#type(),
                    lvalue,
                    rvalue
                ))),
            },
            IOperation::Equal { lhs, rhs } => match (
                evaluate(lhs, environment)?.as_value_ref(),
                evaluate(rhs, environment)?.as_value_ref(),
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
                (lvalue, rvalue) => Err(EvaluationError::G(format!(
                    "invalid equals operation on lhs value type {:?} and rhs value type {:?}: {:?} EQUALS {:?}",
                    lvalue.r#type(),
                    rvalue.r#type(),
                    lvalue,
                    rvalue
                ))),
            },
            IOperation::Index { lhs, rhs } => todo!(),
        },
        AST::SpecialOperation(operation) => match &*operation {
            SpecialOperation::Assign {
                identifier,
                expression,
            } => todo!(),
            SpecialOperation::List(list) => todo!(),
            SpecialOperation::Map(map) => todo!(),
            SpecialOperation::Drop { rhs_iden } => todo!(),
            SpecialOperation::Let {
                identifier,
                r#type,
                expression,
            } => {
                let variable = match (r#type, expression) {
                    (None, None) => Variable::Declared,
                    (Some(r#type), None) => Variable::Typed(*r#type),
                    (None, Some(expression)) => Variable::Valued(
                        evaluate(expression, environment)?.convert_own_life_shared(),
                    ),
                    (Some(r#type), Some(expression)) => {
                        let eval_value = evaluate(expression, environment)?;
                        if eval_value.as_value_ref().r#type() == *r#type {
                            Variable::Valued(eval_value.convert_own_life_shared())
                        } else {
                            return Err(EvaluationError::BindingError(
                                BindingError::LetVariableDifferentType,
                            ));
                        }
                    }
                };
                environment
                    .bindings
                    .declare(TO::as_str(identifier).to_owned(), variable)
                    .map_err(|e| EvaluationError::BindingError(e))?;
                Ok(EvalValue::Owned(Value::UNIT))
            }
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
