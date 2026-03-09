use crate::executor::{
    operation::{Operation, execute_operation},
    variables::{Environment, VariableName, VariableValue},
};

pub enum Expression {
    Variable(VariableName),
    Value(VariableValue),
    Operation(Box<Operation>),
    // Block,
}

pub struct ExpressionExecutionError {
    pub message: String,
}

pub fn execute_expression(
    expression: Expression,
    variable_environment: &mut Environment,
) -> Result<Option<VariableValue>, ExpressionExecutionError> {
    match expression {
        Expression::Value(value) => Ok(Some(value)),
        Expression::Variable(variable_name) => match variable_environment.get(&variable_name) {
            Err(err) => Err(ExpressionExecutionError {
                message: format!("{err:?}"),
            }),
            Ok(value) => Ok(Some(value.clone())),
        },
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
