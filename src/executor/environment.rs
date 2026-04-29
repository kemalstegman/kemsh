use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::executor::concrete::{Concrete, ConcreteKind};

#[derive(Debug)]
pub struct Environment(Vec<Rc<RefCell<HashMap<String, Variable>>>>);

impl Environment {
    pub fn new_with_default_globals() -> Self {
        let mut environment = Self(Vec::new());
        environment.push_scope();
        environment
            .declare(
                String::from("_KEMSH_VERSION"),
                Variable::Initialized(Concrete::String(Rc::from("0.3.1"))),
            )
            .unwrap();
        environment
    }
    pub fn from_weak_ignore_dropped(weak_env: &WeakEnvironment) -> Self {
        Self(weak_env.0.iter().filter_map(|w| w.upgrade()).collect())
    }
    pub fn weak(&self) -> WeakEnvironment {
        WeakEnvironment(Rc::from(
            self.0.iter().map(|r| Rc::downgrade(r)).collect::<Vec<_>>(),
        ))
    }
    pub fn push_scope(&mut self) {
        self.0.push(Rc::new(RefCell::new(HashMap::new())));
    }
    pub fn pop_scope(&mut self) -> Option<Rc<RefCell<HashMap<String, Variable>>>> {
        self.0.pop()
    }
    pub fn declare(&mut self, name: String, variable: Variable) -> Result<(), EnvironmentError> {
        match self.0.last() {
            None => Err(EnvironmentError::NoScopes),
            Some(scope) => {
                if scope.borrow().contains_key(&name) {
                    Err(EnvironmentError::AlreadyDeclared)
                } else {
                    scope.borrow_mut().insert(name, variable);
                    Ok(())
                }
            }
        }
    }
    pub fn undeclare(&mut self, name: &str) -> Result<Option<Concrete>, EnvironmentError> {
        match self.0.last() {
            None => Err(EnvironmentError::NoScopes),
            Some(scope) => match scope.borrow_mut().remove(name) {
                None => Err(EnvironmentError::NotDeclared),
                Some(Variable::Initialized(concrete)) => Ok(Some(concrete)),
                Some(Variable::Typed(_) | Variable::Declared) => Ok(None),
            },
        }
    }
    pub fn get(&self, name: &str) -> Result<Variable, EnvironmentError> {
        for scope in self.0.iter().rev() {
            if let Some(variable) = scope.borrow().get(name) {
                return Ok(variable.clone());
            }
        }
        Err(EnvironmentError::NotDeclared)
    }
    pub fn assign(
        &mut self,
        name: &str,
        concrete: Concrete,
    ) -> Result<Option<Concrete>, EnvironmentError> {
        for scope in self.0.iter().rev() {
            if let Some(variable) = scope.borrow_mut().get_mut(name) {
                return variable.replace(concrete);
            }
        }
        Err(EnvironmentError::NotDeclared)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnvironmentError {
    NoScopes,
    NotDeclared,
    AlreadyDeclared,
    KindNotMatch,
}

#[derive(Debug, Clone)]
pub struct WeakEnvironment(Rc<[Weak<RefCell<HashMap<String, Variable>>>]>);

#[derive(Debug, Clone)]
pub enum Variable {
    Declared,
    Typed(ConcreteKind),
    Initialized(Concrete),
}

impl Variable {
    pub fn concrete(self) -> Option<Concrete> {
        match self {
            Variable::Declared => None,
            Variable::Typed(_) => None,
            Variable::Initialized(concrete) => Some(concrete),
        }
    }
    fn replace(&mut self, mut concrete: Concrete) -> Result<Option<Concrete>, EnvironmentError> {
        match self {
            Variable::Declared => {
                *self = Variable::Initialized(concrete);
                Ok(None)
            }
            Variable::Typed(kind) => {
                if concrete.kind() == *kind {
                    *self = Variable::Initialized(concrete);
                    Ok(None)
                } else {
                    Err(EnvironmentError::KindNotMatch)
                }
            }
            Variable::Initialized(held) => {
                if concrete.kind() == held.kind() {
                    std::mem::swap(held, &mut concrete);
                    Ok(Some(concrete))
                } else {
                    Err(EnvironmentError::KindNotMatch)
                }
            }
        }
    }
}
