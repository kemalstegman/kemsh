mod environment;

use crate::{
    ast::{
        BraceBlock, Concrete, Expression, Operation,
        VoidConcrete::{self, NonVoid},
    },
    executor::environment::{Environment, Variable},
};

pub struct Executor {
    environment: Environment,
}
impl Executor {
    pub fn new() -> Self {
        Self {
            environment: Environment::new_with_default_globals(),
        }
    }
    pub fn execute_top_level_expression(
        &mut self,
        expression: Expression,
    ) -> Result<(), ExecutorError> {
        println!("{:?}", self.execute_expression(expression)?);
        Ok(())
    }
    fn execute_expression(
        &mut self,
        expression: Expression,
    ) -> Result<VoidConcrete, ExecutorError> {
        match expression {
            Expression::Concrete(concrete) => Ok(NonVoid(concrete)),
            Expression::Identifier(identifier) => match self.environment.get(&identifier.0) {
                Err(err) => Err(ExecutorError::Generic { message: err }),
                Ok(variable) => match variable.get() {
                    None => Err(ExecutorError::Generic {
                        message: String::from("variable uninitialized"),
                    }),
                    Some(concrete) => Ok(NonVoid(concrete.clone())),
                },
            },
            Expression::BraceBlock(BraceBlock(expressions, tail_expression)) => {
                self.environment.push_scope();
                for expression in expressions {
                    self.execute_expression(expression)?;
                }
                let expression = if let Some(expression) = tail_expression {
                    self.execute_expression(*expression)?
                } else {
                    VoidConcrete::Void
                };
                self.environment.pop_scope();
                Ok(expression)
            }
            Expression::Operation(operation) => match *operation {
                Operation::Let {
                    lhs,
                    rhs: Some(rhs),
                } => match self.execute_expression(rhs)? {
                    VoidConcrete::Void => Err(ExecutorError::Generic {
                        message: String::from("expected value from expression"),
                    }),
                    VoidConcrete::NonVoid(rhs) => {
                        if let Some(kind) = lhs.type_annotation {
                            if kind != rhs.kind() {
                                return Err(ExecutorError::Generic {
                                    message: String::from("type does not match"),
                                });
                            }
                        }
                        match self
                            .environment
                            .declare(lhs.identifier.0, Variable::Initialized(rhs))
                        {
                            Ok(()) => Ok(VoidConcrete::Void),
                            Err(err) => Err(ExecutorError::Generic { message: err }),
                        }
                    }
                },
                Operation::Let { lhs, rhs: None } => {
                    match self.environment.declare(
                        lhs.identifier.0,
                        match lhs.type_annotation {
                            Some(kind) => Variable::Typed(kind),
                            None => Variable::Declared,
                        },
                    ) {
                        Ok(()) => Ok(VoidConcrete::Void),
                        Err(err) => Err(ExecutorError::Generic { message: err }),
                    }
                }
                Operation::Assign { lhs, rhs } => {
                    if !lhs.indices.is_empty() {
                        todo!()
                    }
                    match self.execute_expression(rhs)? {
                        VoidConcrete::Void => Err(ExecutorError::Generic {
                            message: String::from("expected value from expression"),
                        }),
                        VoidConcrete::NonVoid(rhs) => match self.environment.get(&lhs.identifier.0)
                        {
                            Err(err) => Err(ExecutorError::Generic { message: err }),
                            Ok(variable) => match variable.assign(rhs) {
                                Err(err) => Err(ExecutorError::Generic { message: err }),
                                Ok(_) => Ok(VoidConcrete::Void),
                            },
                        },
                    }
                }
                Operation::AssignVoid { lhs } => match self.environment.void_assign(&lhs.0) {
                    Err(err) => Err(ExecutorError::Generic { message: err }),
                    Ok(_) => Ok(VoidConcrete::Void),
                },
                Operation::AddConcat { lhs, rhs } => {
                    let lhs = self.execute_expression(lhs)?;
                    let rhs = self.execute_expression(rhs)?;
                    match (lhs, rhs) {
                        (NonVoid(Concrete::Integer(lhs)), NonVoid(Concrete::Integer(rhs))) => {
                            Ok(NonVoid(Concrete::Integer(lhs + rhs)))
                        }
                        (NonVoid(Concrete::Float(lhs)), NonVoid(Concrete::Float(rhs))) => {
                            Ok(NonVoid(Concrete::Float(lhs + rhs)))
                        }
                        (NonVoid(Concrete::String(lhs)), NonVoid(Concrete::String(rhs))) => {
                            Ok(NonVoid(Concrete::String(format!("{lhs}{rhs}"))))
                        }
                        (lhs, rhs) => Err(ExecutorError::Generic {
                            message: format!(
                                "invalid + operands {:?} and {:?}",
                                lhs.kind(),
                                rhs.kind()
                            ),
                        }),
                    }
                }
                Operation::Subtract { lhs, rhs } => {
                    let lhs = self.execute_expression(lhs)?;
                    let rhs = self.execute_expression(rhs)?;
                    match (lhs, rhs) {
                        (NonVoid(Concrete::Integer(lhs)), NonVoid(Concrete::Integer(rhs))) => {
                            Ok(NonVoid(Concrete::Integer(lhs - rhs)))
                        }
                        (NonVoid(Concrete::Float(lhs)), NonVoid(Concrete::Float(rhs))) => {
                            Ok(NonVoid(Concrete::Float(lhs - rhs)))
                        }
                        (lhs, rhs) => Err(ExecutorError::Generic {
                            message: format!(
                                "invalid - operands {:?} and {:?}",
                                lhs.kind(),
                                rhs.kind()
                            ),
                        }),
                    }
                }
                Operation::Multiply { lhs, rhs } => {
                    let lhs = self.execute_expression(lhs)?;
                    let rhs = self.execute_expression(rhs)?;
                    match (lhs, rhs) {
                        (NonVoid(Concrete::Integer(lhs)), NonVoid(Concrete::Integer(rhs))) => {
                            Ok(NonVoid(Concrete::Integer(lhs * rhs)))
                        }
                        (NonVoid(Concrete::Float(lhs)), NonVoid(Concrete::Float(rhs))) => {
                            Ok(NonVoid(Concrete::Float(lhs * rhs)))
                        }
                        (lhs, rhs) => Err(ExecutorError::Generic {
                            message: format!(
                                "invalid * operands {:?} and {:?}",
                                lhs.kind(),
                                rhs.kind()
                            ),
                        }),
                    }
                }
                Operation::Divide { lhs, rhs } => {
                    let lhs = self.execute_expression(lhs)?;
                    let rhs = self.execute_expression(rhs)?;
                    match (lhs, rhs) {
                        (NonVoid(Concrete::Integer(lhs)), NonVoid(Concrete::Integer(rhs))) => {
                            Ok(NonVoid(Concrete::Integer(lhs / rhs)))
                        }
                        (NonVoid(Concrete::Float(lhs)), NonVoid(Concrete::Float(rhs))) => {
                            Ok(NonVoid(Concrete::Float(lhs / rhs)))
                        }
                        (lhs, rhs) => Err(ExecutorError::Generic {
                            message: format!(
                                "invalid / operands {:?} and {:?}",
                                lhs.kind(),
                                rhs.kind()
                            ),
                        }),
                    }
                }
                Operation::Modulo { lhs, rhs } => {
                    let lhs = self.execute_expression(lhs)?;
                    let rhs = self.execute_expression(rhs)?;
                    match (lhs, rhs) {
                        (NonVoid(Concrete::Integer(lhs)), NonVoid(Concrete::Integer(rhs))) => {
                            Ok(NonVoid(Concrete::Integer(lhs % rhs)))
                        }
                        (NonVoid(Concrete::Float(lhs)), NonVoid(Concrete::Float(rhs))) => {
                            Ok(NonVoid(Concrete::Float(lhs % rhs)))
                        }
                        (lhs, rhs) => Err(ExecutorError::Generic {
                            message: format!(
                                "invalid % operands {:?} and {:?}",
                                lhs.kind(),
                                rhs.kind()
                            ),
                        }),
                    }
                }
                _ => todo!(),
            },
        }
    }
}

#[derive(Debug)]
pub enum ExecutorError {
    Generic { message: String },
}
