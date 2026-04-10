use std::collections::HashMap;

use crate::ast::{Concrete, ConcreteKind};

pub enum Variable {
    Declared,
    Typed(ConcreteKind),
    Initialized(Concrete),
}

impl Variable {
    pub fn get(&mut self) -> Option<&mut Concrete> {
        match self {
            Variable::Declared => None,
            Variable::Typed(_) => None,
            Variable::Initialized(concrete) => Some(concrete),
        }
    }
    pub fn assign(&mut self, mut concrete: Concrete) -> Result<Option<Concrete>, String> {
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
                    Err(String::from("type does not match"))
                }
            }
            Variable::Initialized(held) => {
                if concrete.kind() == held.kind() {
                    std::mem::swap(held, &mut concrete);
                    Ok(Some(concrete))
                } else {
                    Err(String::from("type does not match"))
                }
            }
        }
    }
}

pub struct Environment(Vec<HashMap<String, Variable>>);

impl Environment {
    pub fn new_with_default_globals() -> Self {
        let mut environment = Self::new_no_scopes();
        environment.push_scope();
        environment
            .declare(
                String::from("_KEMSH_VERSION"),
                Variable::Initialized(Concrete::String(String::from("0.2.0"))),
            )
            .unwrap();
        environment
    }
    fn new_no_scopes() -> Self {
        Self(Vec::new())
    }
    pub fn push_scope(&mut self) {
        self.0.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) {
        self.0.pop();
    }
    pub fn declare(&mut self, name: String, variable: Variable) -> Result<(), String> {
        match self.0.last_mut() {
            None => Err(String::from("no scopes")),
            Some(scope) => {
                if scope.contains_key(&name) {
                    Err(String::from("already declared"))
                } else {
                    scope.insert(name, variable);
                    Ok(())
                }
            }
        }
    }
    // pub fn assign(
    //     &mut self,
    //     name: &String,
    //     concrete: Concrete,
    // ) -> Result<Option<Concrete>, String> {
    //     for scope in self.0.iter_mut().rev() {
    //         if let Some(variable) = scope.get_mut(name) {
    //             return variable.assign(concrete);
    //         }
    //     }
    //     Err(String::from("variable not declared"))
    // }
    pub fn void_assign(&mut self, name: &str) -> Result<Option<Concrete>, String> {
        match self.0.last_mut() {
            None => Err(String::from("no scopes")),
            Some(scope) => match scope.remove(name) {
                None => Err(String::from("not declared")),
                Some(Variable::Initialized(concrete)) => Ok(Some(concrete)),
                Some(Variable::Typed(_) | Variable::Declared) => Ok(None),
            },
        }
    }
    pub fn get(&mut self, name: &str) -> Result<&mut Variable, String> {
        for scope in self.0.iter_mut().rev() {
            if let Some(variable) = scope.get_mut(name) {
                return Ok(variable);
            }
        }
        Err(String::from("variable not declared"))
    }
}
