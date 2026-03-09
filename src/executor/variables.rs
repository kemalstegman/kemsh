use std::collections::HashMap;

pub type VariableName = String;

#[derive(Debug, Clone)]
pub enum VariableValue {
    LiteralString(String),
    LiteralInteger(i64),
    LiteralFloating(f64),
    LiteralBoolean(bool),
}

impl VariableValue {
    pub fn kind(&self) -> VariableKind {
        match self {
            VariableValue::LiteralString(_) => VariableKind::LiteralString,
            VariableValue::LiteralInteger(_) => VariableKind::LiteralInteger,
            VariableValue::LiteralFloating(_) => VariableKind::LiteralFloating,
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
enum Variable {
    DeclaredTypeless,
    DeclaredTyped(VariableKind),
    Initialized(VariableValue),
}

impl Variable {
    fn assign(&mut self, mut value: VariableValue) -> Result<Option<VariableValue>, ()> {
        match self {
            Variable::DeclaredTypeless => {
                *self = Variable::Initialized(value);
                Ok(None)
            }
            Variable::DeclaredTyped(kind) => {
                if value.kind() == *kind {
                    *self = Variable::Initialized(value);
                    Ok(None)
                } else {
                    Err(())
                }
            }
            Variable::Initialized(held) => {
                if value.kind() == held.kind() {
                    std::mem::swap(held, &mut value);
                    Ok(Some(value))
                } else {
                    Err(())
                }
            }
        }
    }
    fn get(&self) -> Option<&VariableValue> {
        match self {
            Variable::DeclaredTypeless | Variable::DeclaredTyped(_) => None,
            Variable::Initialized(value) => Some(value),
        }
    }
}

#[derive(Debug)]
pub struct EnvironmentError {
    pub message: String,
}

pub struct Environment(Vec<HashMap<VariableName, Variable>>);

impl Environment {
    fn new_no_scopes() -> Self {
        Self(Vec::new())
    }
    pub fn new_with_default_globals() -> Self {
        let mut environment = Self::new_no_scopes();
        environment.push_scope();
        environment
            .declare_initialized(
                "_KEMSH_VERSION".to_string(),
                VariableValue::LiteralString("0.1.0".to_string()),
            )
            .unwrap();
        environment
    }
    pub fn push_scope(&mut self) {
        self.0.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        self.0.pop();
    }
    pub fn get(&self, name: &VariableName) -> Result<&VariableValue, EnvironmentError> {
        for scope in self.0.iter().rev() {
            match scope.get(name) {
                None => continue,
                Some(variable) => match variable.get() {
                    None => {
                        return Err(EnvironmentError {
                            message: "variable is not initialized".to_string(),
                        });
                    }
                    Some(value) => return Ok(value),
                },
            }
        }
        Err(EnvironmentError {
            message: "variable does not exist".to_string(),
        })
    }
    pub fn assign(
        &mut self,
        name: &VariableName,
        value: VariableValue,
    ) -> Result<(), EnvironmentError> {
        for scope in self.0.iter_mut().rev() {
            match scope.get_mut(name) {
                None => continue,
                Some(variable) => match variable.assign(value) {
                    Err(()) => {
                        return Err(EnvironmentError {
                            message: "variable type does not match value".to_string(),
                        });
                    }
                    Ok(None) | Ok(Some(_)) => return Ok(()),
                },
            }
        }
        Err(EnvironmentError {
            message: "variable does not exist".to_string(),
        })
    }
    fn declare(&mut self, name: VariableName, variable: Variable) -> Result<(), EnvironmentError> {
        match self.0.last_mut() {
            None => {
                return Err(EnvironmentError {
                    message: "environment has no scopes".to_string(),
                });
            }
            Some(scope) => match scope.contains_key(&name) {
                true => {
                    return Err(EnvironmentError {
                        message: "variable already declared".to_string(),
                    });
                }
                false => {
                    scope.insert(name, variable);
                    Ok(())
                }
            },
        }
    }
    pub fn declare_typeless(&mut self, name: VariableName) -> Result<(), EnvironmentError> {
        self.declare(name, Variable::DeclaredTypeless)
    }
    pub fn declare_typed(
        &mut self,
        name: VariableName,
        kind: VariableKind,
    ) -> Result<(), EnvironmentError> {
        self.declare(name, Variable::DeclaredTyped(kind))
    }
    pub fn declare_initialized(
        &mut self,
        name: VariableName,
        value: VariableValue,
    ) -> Result<(), EnvironmentError> {
        self.declare(name, Variable::Initialized(value))
    }
}
