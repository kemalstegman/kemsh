//! todo!() working directory

mod environment;

pub mod concrete;

use std::{
    env::{current_dir, set_current_dir},
    process::Command,
    rc::Rc,
};

use crate::{
    ast::{BraceBlock, Expression, Literal, Operation},
    executor::{
        concrete::{Concrete, VoidConcrete},
        environment::{Environment, EnvironmentError, Variable},
    },
};

pub struct Executor {
    environment: Environment,
    working_directory: Rc<str>,
}
impl Executor {
    pub fn new() -> Self {
        Self {
            environment: Environment::new_with_default_globals(),
            working_directory: Rc::from(
                current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            ),
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
            Expression::Literal(literal) => Ok(VoidConcrete::Rife(match literal {
                Literal::Boolean(b) => Concrete::Boolean(b),
                Literal::Float(f) => Concrete::Float(f),
                Literal::Integer(i) => Concrete::Integer(i),
                Literal::RawString(s) => Concrete::String(Rc::from(s)),
            })),
            Expression::CompoundLiteral(_) => todo!(),
            Expression::Identifier(identifier) => match self.environment.get(&identifier.0) {
                Err(err) => Err(ExecutorError::Environment(err)),
                Ok(variable) => match variable.concrete() {
                    None => Err(ExecutorError::Generic {
                        message: String::from("variable uninitialized"),
                    }),
                    Some(concrete) => Ok(VoidConcrete::Rife(concrete)),
                },
            },
            Expression::BraceBlock(BraceBlock {
                expressions,
                evaluate_to_tail_expression,
            }) => {
                self.environment.push_scope();
                let result = (|| {
                    let mut last = VoidConcrete::Void;
                    for expression in expressions {
                        last = self.execute_expression(expression)?;
                    }
                    if evaluate_to_tail_expression {
                        Ok(last)
                    } else {
                        Ok(VoidConcrete::Void)
                    }
                })();
                self.environment.pop_scope();
                result
            }
            Expression::Operation(operation) => match *operation {
                Operation::Let {
                    lhs,
                    rhs: Some(rhs),
                } => match self.execute_expression(rhs)? {
                    VoidConcrete::Void => Err(ExecutorError::Generic {
                        message: String::from("expected value from expression"),
                    }),
                    VoidConcrete::Rife(rhs) => {
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
                            Err(err) => Err(ExecutorError::Environment(err)),
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
                        Err(err) => Err(ExecutorError::Environment(err)),
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
                        VoidConcrete::Rife(rhs) => {
                            match self.environment.assign(&lhs.identifier.0, rhs) {
                                Err(err) => Err(ExecutorError::Environment(err)),
                                Ok(_) => Ok(VoidConcrete::Void),
                            }
                        }
                    }
                }
                Operation::AssignVoid { lhs } => match self.environment.undeclare(&lhs.0) {
                    Err(err) => Err(ExecutorError::Environment(err)),
                    Ok(_) => Ok(VoidConcrete::Void),
                },
                Operation::AddConcat { lhs, rhs } => {
                    let lhs = self.execute_expression(lhs)?;
                    let rhs = self.execute_expression(rhs)?;
                    match (lhs, rhs) {
                        (
                            VoidConcrete::Rife(Concrete::Integer(lhs)),
                            VoidConcrete::Rife(Concrete::Integer(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Integer(lhs + rhs))),
                        (
                            VoidConcrete::Rife(Concrete::Float(lhs)),
                            VoidConcrete::Rife(Concrete::Float(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Float(lhs + rhs))),
                        (
                            VoidConcrete::Rife(Concrete::String(lhs)),
                            VoidConcrete::Rife(Concrete::String(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::String(Rc::from(format!(
                            "{lhs}{rhs}"
                        ))))),
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
                        (
                            VoidConcrete::Rife(Concrete::Integer(lhs)),
                            VoidConcrete::Rife(Concrete::Integer(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Integer(lhs - rhs))),
                        (
                            VoidConcrete::Rife(Concrete::Float(lhs)),
                            VoidConcrete::Rife(Concrete::Float(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Float(lhs - rhs))),
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
                        (
                            VoidConcrete::Rife(Concrete::Integer(lhs)),
                            VoidConcrete::Rife(Concrete::Integer(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Integer(lhs * rhs))),
                        (
                            VoidConcrete::Rife(Concrete::Float(lhs)),
                            VoidConcrete::Rife(Concrete::Float(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Float(lhs * rhs))),
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
                        (
                            VoidConcrete::Rife(Concrete::Integer(lhs)),
                            VoidConcrete::Rife(Concrete::Integer(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Integer(lhs / rhs))),
                        (
                            VoidConcrete::Rife(Concrete::Float(lhs)),
                            VoidConcrete::Rife(Concrete::Float(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Float(lhs / rhs))),
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
                        (
                            VoidConcrete::Rife(Concrete::Integer(lhs)),
                            VoidConcrete::Rife(Concrete::Integer(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Integer(lhs % rhs))),
                        (
                            VoidConcrete::Rife(Concrete::Float(lhs)),
                            VoidConcrete::Rife(Concrete::Float(rhs)),
                        ) => Ok(VoidConcrete::Rife(Concrete::Float(lhs % rhs))),
                        (lhs, rhs) => Err(ExecutorError::Generic {
                            message: format!(
                                "invalid % operands {:?} and {:?}",
                                lhs.kind(),
                                rhs.kind()
                            ),
                        }),
                    }
                }
                Operation::Exit(expression) => match self.execute_expression(expression)? {
                    VoidConcrete::Rife(Concrete::Integer(st)) => Err(ExecutorError::Exit(st)),
                    conc => Err(ExecutorError::Generic {
                        message: format!("invalid exit operand {:?}", conc.kind()),
                    }),
                },
                Operation::ChangeDirectory(expression) => {
                    match self.execute_expression(expression)? {
                        VoidConcrete::Rife(Concrete::String(s)) => match set_current_dir(&*s) {
                            Ok(()) => {
                                let previous_working_directory = Rc::clone(&self.working_directory);
                                self.working_directory = Rc::clone(&s);
                                Ok(VoidConcrete::Rife(Concrete::String(
                                    previous_working_directory,
                                )))
                            }
                            Err(_) => todo!(),
                        },
                        conc => Err(ExecutorError::Generic {
                            message: format!("invalid change directory operand {:?}", conc.kind()),
                        }),
                    }
                }
                Operation::Run(expression) => match self.execute_expression(expression)? {
                    // todo!()
                    VoidConcrete::Rife(Concrete::String(s)) => {
                        Command::new(&*s).spawn().unwrap().wait().unwrap();
                        Ok(VoidConcrete::Void)
                    }
                    conc => Err(ExecutorError::Generic {
                        message: format!("invalid run operand {:?}", conc.kind()),
                    }),
                },
                Operation::Spawn(expression) => match self.execute_expression(expression)? {
                    // todo!()
                    // VoidConcrete::Rife(Concrete::String(s)) => {
                    //     #[cfg(windows)]
                    //     Command::new(s).creation_flags(0x00000010).spawn().unwrap(); // CREATE_NEW_CONSOLE
                    //     #[cfg(unix)]
                    //     Command::new(s).spawn().unwrap();
                    //     Ok(VoidConcrete::Void)
                    // }
                    _ => todo!(),
                },
                _ => todo!(),
            },
        }
    }
}

#[derive(Debug)]
pub enum ExecutorError {
    Environment(EnvironmentError),
    Generic { message: String },
    Exit(i64),
}
