use std::collections::HashMap;

use crate::executor::variable_environment::VariableEnvironment;

mod variable_environment;

pub fn new_variable_environment_with_globals() -> VariableEnvironment<VariableValue> {
    let mut ve = VariableEnvironment::new();
    ve.push_scope();
    ve.declare_and_initialize_variable(
        "_KEMSH_VERSION".to_string(),
        VariableValue::LiteralString("0.1.0".to_string()),
    );
    ve
}

#[derive(Debug, Clone)]
pub enum VariableValue {
    LiteralString(String),
    LiteralInteger(i64),
    LiteralFloating(f64),
    LiteralBoolean(bool),
}

pub enum Expression {
    Value(VariableValue),
    Variable(String),
    Operation(Box<Operation>),
    // Block,
}

pub enum Operation {
    Add { lhs: Expression, rhs: Expression },
    Subtract { lhs: Expression, rhs: Expression },
    Multiply { lhs: Expression, rhs: Expression },
    Divide { lhs: Expression, rhs: Expression },
}

pub enum Instruction {
    Let(LetInstruction),
    Set(SetInstruction),
    // ChangeDirectory(ChangeDirectoryInstruction),
    Echo(EchoInstruction),
    // Spawn(SpawnInstruction),
}

pub struct LetInstruction {
    pub variable_name: String,
    // pub variable_type: (),
    pub expression: Option<Expression>,
}
pub struct SetInstruction {
    pub variable_name: String,
    pub expression: Expression,
}

// pub struct ChangeDirectoryInstruction;
pub struct EchoInstruction {
    pub expression: Expression,
}

// todo!() variants of process execution
// pub struct SpawnInstruction;

#[derive(Debug)]
pub struct InstructionExecutionError {
    pub message: String,
}

pub fn execute_instruction(
    instruction: Instruction,
    variable_environment: &mut VariableEnvironment<VariableValue>,
) -> Result<(), InstructionExecutionError> {
    match instruction {
        Instruction::Let(LetInstruction {
            variable_name,
            expression: None,
        }) => {
            variable_environment.declare_variable(variable_name);
        }
        Instruction::Let(LetInstruction {
            variable_name,
            expression: Some(expression),
        }) => match execute_expression(expression, variable_environment) {
            Err(e) => Err(InstructionExecutionError {
                message: format!("Error executing expression: {}", e.message),
            })?,
            Ok(None) => Err(InstructionExecutionError {
                message: "Expected a value from expression".to_string(),
            })?,
            Ok(Some(variable_value)) => {
                variable_environment.declare_and_initialize_variable(variable_name, variable_value);
            }
        },
        Instruction::Set(SetInstruction {
            variable_name,
            expression,
        }) => match execute_expression(expression, variable_environment) {
            Err(e) => Err(InstructionExecutionError {
                message: format!("Error executing expression: {}", e.message),
            })?,
            Ok(None) => Err(InstructionExecutionError {
                message: "Expected a value from expression".to_string(),
            })?,
            Ok(Some(variable_value)) => {
                match variable_environment.set_variable(&variable_name, variable_value) {
                    Err(err) => Err(InstructionExecutionError {
                        message: format!("{err:?}"),
                    })?,
                    Ok(_) => (),
                }
            }
        },
        Instruction::Echo(EchoInstruction { expression }) => {
            match execute_expression(expression, variable_environment) {
                Err(e) => Err(InstructionExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(InstructionExecutionError {
                    message: "Expected a value from expression".to_string(),
                })?,
                Ok(Some(value)) => {
                    println!("{value:?}");
                }
            }
        }
    }
    Ok(())
}

struct ExpressionExecutionError {
    message: String,
}

fn execute_expression(
    expression: Expression,
    variable_environment: &mut VariableEnvironment<VariableValue>,
) -> Result<Option<VariableValue>, ExpressionExecutionError> {
    match expression {
        Expression::Value(value) => Ok(Some(value)),
        Expression::Variable(variable_name) => {
            match variable_environment.get_variable(&variable_name) {
                Ok(None) => Err(ExpressionExecutionError {
                    message: "Variable dne".to_string(),
                }),
                Err(err) => Err(ExpressionExecutionError {
                    message: format!("{err:?}"),
                }),
                Ok(Some(value)) => Ok(Some(value.clone())),
            }
        }
        Expression::Operation(operation) => {
            match execute_operation(*operation, variable_environment) {
                Ok(value) => Ok(Some(value)),
                Err(e) => Err(ExpressionExecutionError {
                    message: format!("Error executing operation: {}", e.message),
                }),
            }
        }
    }
}

struct OperationExecutionError {
    message: String,
}

fn execute_operation(
    operation: Operation,
    variable_environment: &mut VariableEnvironment<VariableValue>,
) -> Result<VariableValue, OperationExecutionError> {
    match operation {
        Operation::Add { lhs, rhs } => {
            let lhs_eval = match execute_expression(lhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
                    message: "Expected a value from expression".to_string(),
                })?,
                Ok(Some(value)) => value,
            };
            let rhs_eval = match execute_expression(rhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
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
            let lhs_eval = match execute_expression(lhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
                    message: "Expected a value from expression".to_string(),
                })?,
                Ok(Some(value)) => value,
            };
            let rhs_eval = match execute_expression(rhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
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
            let lhs_eval = match execute_expression(lhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
                    message: "Expected a value from expression".to_string(),
                })?,
                Ok(Some(value)) => value,
            };
            let rhs_eval = match execute_expression(rhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
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
            let lhs_eval = match execute_expression(lhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
                    message: "Expected a value from expression".to_string(),
                })?,
                Ok(Some(value)) => value,
            };
            let rhs_eval = match execute_expression(rhs, variable_environment) {
                Err(e) => Err(OperationExecutionError {
                    message: format!("Error executing expression: {}", e.message),
                })?,
                Ok(None) => Err(OperationExecutionError {
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

#[test]
fn test_execution() {
    let mut ve = VariableEnvironment::new();
    ve.push_scope();
    let instructions = [
        Instruction::Let(LetInstruction {
            variable_name: "x".to_string(),
            expression: Some(Expression::Value(VariableValue::LiteralInteger(12))),
        }),
        Instruction::Echo(EchoInstruction {
            expression: Expression::Variable("x".to_string()),
        }),
        Instruction::Set(SetInstruction {
            variable_name: "x".to_string(),
            expression: Expression::Operation(Box::new(Operation::Add {
                lhs: Expression::Variable("x".to_string()),
                rhs: Expression::Value(VariableValue::LiteralInteger(1)),
            })),
        }),
        Instruction::Echo(EchoInstruction {
            expression: Expression::Variable("x".to_string()),
        }),
        Instruction::Set(SetInstruction {
            variable_name: "x".to_string(),
            expression: Expression::Operation(Box::new(Operation::Multiply {
                lhs: Expression::Variable("x".to_string()),
                rhs: Expression::Value(VariableValue::LiteralInteger(3)),
            })),
        }),
        Instruction::Echo(EchoInstruction {
            expression: Expression::Variable("x".to_string()),
        }),
    ];
    for i in instructions {
        execute_instruction(i, &mut ve).unwrap();
    }
}
