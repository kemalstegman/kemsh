use crate::executor::{
    expression::{Expression, execute_expression},
    variables::{Environment, VariableValue},
};

pub enum Operation {
    Add { lhs: Expression, rhs: Expression },
    Subtract { lhs: Expression, rhs: Expression },
    Multiply { lhs: Expression, rhs: Expression },
    Divide { lhs: Expression, rhs: Expression },
}

pub struct OperationExecutionError {
    pub message: String,
}

pub fn execute_operation(
    operation: Operation,
    variable_environment: &mut Environment,
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
