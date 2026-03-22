use crate::abstract_syntax_tree::{
    ChangeDirectoryInstruction, EchoInstruction, ExitInstruction, Expression, Instruction,
    LetInstruction, Operation, SetInstruction, VariableValue,
};

mod variable_environment;
use variable_environment::Environment;

#[derive(Debug)]
pub struct ExecutionError {
    message: String,
}

pub struct Executor {
    variable_environment: Environment,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            variable_environment: Environment::new_with_default_globals(),
        }
    }
    pub fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), ExecutionError> {
        match instruction {
            Instruction::Let(LetInstruction {
                variable_name,
                variable_kind: None,
                expression: None,
            }) => self.variable_environment.declare_typeless(variable_name),
            Instruction::Let(LetInstruction {
                variable_name,
                variable_kind: Some(variable_kind),
                expression: None,
            }) => self
                .variable_environment
                .declare_typed(variable_name, variable_kind),
            Instruction::Let(LetInstruction {
                variable_name,
                variable_kind: None,
                expression: Some(expression),
            }) => match self.execute_expression(expression)? {
                None => Err(ExecutionError {
                    message: String::from("expected value from expression"),
                }),
                Some(variable_value) => self
                    .variable_environment
                    .declare_initialized(variable_name, variable_value),
            },
            Instruction::Let(LetInstruction {
                variable_name,
                variable_kind: Some(variable_kind),
                expression: Some(expression),
            }) => match self.execute_expression(expression)? {
                None => Err(ExecutionError {
                    message: String::from("expected value from expression"),
                }),
                Some(variable_value) => {
                    if variable_value.kind() == variable_kind {
                        self.variable_environment
                            .declare_initialized(variable_name, variable_value)
                    } else {
                        Err(ExecutionError {
                            message: format!(
                                "expected {:?} from expression, got {:?}",
                                variable_kind,
                                variable_value.kind()
                            ),
                        })
                    }
                }
            },
            Instruction::Set(SetInstruction {
                variable_name,
                expression,
            }) => match self.execute_expression(expression)? {
                None => Err(ExecutionError {
                    message: String::from("expected value from expression"),
                }),
                Some(variable_value) => self
                    .variable_environment
                    .assign(&variable_name, variable_value),
            },
            Instruction::ChangeDirectory(ChangeDirectoryInstruction { expression }) => {
                match self.execute_expression(expression)? {
                    None => Err(ExecutionError {
                        message: String::from("expected value from expression"),
                    }),
                    Some(VariableValue::LiteralString(string)) => {
                        // todo!() this needs to be redone
                        match std::env::set_current_dir(&string) {
                            Ok(()) => Ok(()),
                            Err(err) => Err(ExecutionError {
                                message: format!("error changing directory: {:?}", err),
                            }),
                        }
                    }
                    Some(value) => Err(ExecutionError {
                        message: format!("expected string from expression, got {:?}", value.kind()),
                    }),
                }
            }
            Instruction::Echo(EchoInstruction { expressions }) => {
                let _ = std::io::stdout().lock();
                for (i, expression) in expressions.into_iter().enumerate() {
                    match self.execute_expression(expression)? {
                        None => {
                            return Err(ExecutionError {
                                message: String::from("expected value from expression"),
                            });
                        }
                        Some(value) => {
                            if i != 0 {
                                print!(" ");
                            }
                            print!("{value:?}");
                        }
                    }
                }
                print!("\n");
                Ok(())
            }
            Instruction::Exit(ExitInstruction { expression }) => {
                match self.execute_expression(expression)? {
                    None => Err(ExecutionError {
                        message: String::from("expected value from expression"),
                    }),
                    Some(VariableValue::LiteralInteger(integer)) => {
                        std::process::exit(integer as i32);
                    }
                    Some(value) => Err(ExecutionError {
                        message: format!(
                            "expected integer from expression, got {:?}",
                            value.kind()
                        ),
                    }),
                }
            }
        }
    }
    fn execute_expression(
        &mut self,
        expression: Expression,
    ) -> Result<Option<VariableValue>, ExecutionError> {
        match expression {
            Expression::Value(value) => Ok(Some(value)),
            Expression::Variable(variable_name) => {
                Ok(Some(self.variable_environment.get(&variable_name)?.clone()))
            }
            Expression::Operation(operation) => Ok(Some(self.execute_operation(*operation)?)),
        }
    }
    fn execute_operation(&mut self, operation: Operation) -> Result<VariableValue, ExecutionError> {
        match operation {
            Operation::Add { lhs, rhs } => {
                let lhs_eval = match self.execute_expression(lhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                let rhs_eval = match self.execute_expression(rhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                match (lhs_eval, rhs_eval) {
                    (VariableValue::LiteralInteger(li), VariableValue::LiteralInteger(ri)) => {
                        Ok(VariableValue::LiteralInteger(li + ri))
                    }
                    _ => todo!(),
                }
            }
            Operation::Subtract { lhs, rhs } => {
                let lhs_eval = match self.execute_expression(lhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                let rhs_eval = match self.execute_expression(rhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                match (lhs_eval, rhs_eval) {
                    (VariableValue::LiteralInteger(li), VariableValue::LiteralInteger(ri)) => {
                        Ok(VariableValue::LiteralInteger(li - ri))
                    }
                    _ => todo!(),
                }
            }
            Operation::Multiply { lhs, rhs } => {
                let lhs_eval = match self.execute_expression(lhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                let rhs_eval = match self.execute_expression(rhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                match (lhs_eval, rhs_eval) {
                    (VariableValue::LiteralInteger(li), VariableValue::LiteralInteger(ri)) => {
                        Ok(VariableValue::LiteralInteger(li * ri))
                    }
                    _ => todo!(),
                }
            }
            Operation::Divide { lhs, rhs } => {
                let lhs_eval = match self.execute_expression(lhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                let rhs_eval = match self.execute_expression(rhs) {
                    Err(e) => Err(ExecutionError {
                        message: format!("Error executing expression: {}", e.message),
                    })?,
                    Ok(None) => Err(ExecutionError {
                        message: "Expected a value from expression".to_string(),
                    })?,
                    Ok(Some(value)) => value,
                };
                match (lhs_eval, rhs_eval) {
                    (VariableValue::LiteralInteger(li), VariableValue::LiteralInteger(ri)) => {
                        Ok(VariableValue::LiteralInteger(li / ri))
                    }
                    _ => todo!(),
                }
            }
        }
    }
}

// #[test]
// fn test_execution() {
//     use expression::Expression;
//     use instruction::{
//         EchoInstruction, Instruction, LetInstruction, SetInstruction, execute_instruction,
//     };
//     use operation::Operation;
//     use variables::{Environment, VariableValue};
//     let mut ve = Environment::new_with_default_globals();
//     ve.push_scope();
//     let instructions = [
//         Instruction::Let(LetInstruction {
//             variable_name: "x".to_string(),
//             variable_kind: None,
//             expression: Some(Expression::Value(VariableValue::LiteralInteger(12))),
//         }),
//         Instruction::Echo(EchoInstruction {
//             expressions: vec![Expression::Variable("x".to_string())],
//         }),
//         Instruction::Set(SetInstruction {
//             variable_name: "x".to_string(),
//             expression: Expression::Operation(Box::new(Operation::Add {
//                 lhs: Expression::Variable("x".to_string()),
//                 rhs: Expression::Value(VariableValue::LiteralInteger(1)),
//             })),
//         }),
//         Instruction::Echo(EchoInstruction {
//             expressions: vec![Expression::Variable("x".to_string())],
//         }),
//         Instruction::Set(SetInstruction {
//             variable_name: "x".to_string(),
//             expression: Expression::Operation(Box::new(Operation::Multiply {
//                 lhs: Expression::Variable("x".to_string()),
//                 rhs: Expression::Value(VariableValue::LiteralInteger(3)),
//             })),
//         }),
//         Instruction::Echo(EchoInstruction {
//             expressions: vec![Expression::Variable("x".to_string())],
//         }),
//     ];
//     for i in instructions {
//         execute_instruction(i, &mut ve).unwrap();
//     }
// }
