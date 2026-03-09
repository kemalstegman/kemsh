use std::collections::HashMap;

pub struct VariableEnvironment<V> {
    scopes: Vec<HashMap<String, Option<V>>>,
}

impl<V> VariableEnvironment<V> {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) -> Option<HashMap<String, Option<V>>> {
        self.scopes.pop()
    }
}

#[derive(Debug)]
pub enum VariableEnvironmentError {
    AlreadyDeclared,
    DoesNotExist,
    Uninitialized,
    ValueTypeMismatch,
    NoScopes,
}

impl<V> VariableEnvironment<V> {
    pub fn declare_variable(&mut self, name: String) -> Result<(), VariableEnvironmentError> {
        match self.scopes.last_mut() {
            None => Err(VariableEnvironmentError::NoScopes)?,
            Some(scope) => match scope.contains_key(&name) {
                true => Err(VariableEnvironmentError::AlreadyDeclared)?,
                false => {
                    scope.insert(name, None);
                    Ok(())
                }
            },
        }
    }
    pub fn declare_and_initialize_variable(
        &mut self,
        name: String,
        value: V,
    ) -> Result<(), VariableEnvironmentError> {
        match self.scopes.last_mut() {
            None => Err(VariableEnvironmentError::NoScopes)?,
            Some(scope) => match scope.contains_key(&name) {
                true => Err(VariableEnvironmentError::AlreadyDeclared)?,
                false => {
                    scope.insert(name, Some(value));
                    Ok(())
                }
            },
        }
    }
    pub fn set_variable(
        &mut self,
        name: &String,
        value: V,
    ) -> Result<Option<V>, VariableEnvironmentError> {
        if self.scopes.len() == 0 {
            Err(VariableEnvironmentError::NoScopes)?
        }
        for scope in self.scopes.iter_mut().rev() {
            if let Some(variable) = scope.get_mut(name) {
                let old_value = variable.take();
                value.
                *variable = Some(value);
                return Ok(old_value);
            }
        }
        Err(VariableEnvironmentError::DoesNotExist)
    }
    pub fn get_variable(&self, name: &String) -> Result<&V, VariableEnvironmentError> {
        if self.scopes.len() == 0 {
            Err(VariableEnvironmentError::NoScopes)?
        }
        for scope in self.scopes.iter().rev() {
            match scope.get(name) {
                Some(Some(value)) => return Ok(value),
                Some(None) => Err(VariableEnvironmentError::Uninitialized)?,
                None => (),
            }
        }
        Err(VariableEnvironmentError::DoesNotExist)
    }
}
