use std::{collections::HashMap, rc::Rc};

use crate::{ast::Expression, executor::environment::WeakEnvironment};

#[derive(Debug, Clone)]
pub enum Concrete {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(Rc<str>),
    List(Rc<Vec<Concrete>>),
    Map(Rc<HashMap<String, Concrete>>),
    Option(Option<Box<Concrete>>),
    Result(Result<Box<Concrete>, Box<Concrete>>),
    Function {
        scopes: WeakEnvironment,
        expressions: Rc<[Expression]>,
        evaluate_to_tail_expression: bool,
    },
    JCmd(()),
    JHandle(()),
    JExit(()),
    FHandle(()),
}

impl Concrete {
    pub fn kind(&self) -> ConcreteKind {
        match self {
            Concrete::Integer(_) => ConcreteKind::Integer,
            Concrete::Float(_) => ConcreteKind::Float,
            Concrete::Boolean(_) => ConcreteKind::Boolean,
            Concrete::String(_) => ConcreteKind::String,
            Concrete::List(_) => ConcreteKind::List,
            Concrete::Map(_) => ConcreteKind::Map,
            Concrete::Option(_) => ConcreteKind::Option,
            Concrete::Result(_) => ConcreteKind::Result,
            Concrete::Function {
                scopes: _,
                expressions: _,
                evaluate_to_tail_expression: _,
            } => ConcreteKind::Function,
            Concrete::JCmd(_) => ConcreteKind::JCmd,
            Concrete::JHandle(_) => ConcreteKind::JHandle,
            Concrete::JExit(_) => ConcreteKind::JExit,
            Concrete::FHandle(_) => ConcreteKind::FHandle,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcreteKind {
    Integer,
    Float,
    Boolean,
    String,
    List,
    Map,
    Option,
    Result,
    Function,
    JCmd,
    JHandle,
    JExit,
    FHandle,
}

#[derive(Debug, Clone)]
pub enum VoidConcrete {
    Void,
    Rife(Concrete),
}

impl VoidConcrete {
    pub fn kind(&self) -> VoidConcreteKind {
        match self {
            VoidConcrete::Void => VoidConcreteKind::Void,
            VoidConcrete::Rife(concrete) => VoidConcreteKind::Rife(concrete.kind()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VoidConcreteKind {
    Void,
    Rife(ConcreteKind),
}
