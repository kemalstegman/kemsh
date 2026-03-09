use crate::executor::{
    expression::{Expression, execute_expression},
    variables::{Environment, VariableName},
};

pub enum Instruction {
    Let(LetInstruction),
    Set(SetInstruction),
    // ChangeDirectory(ChangeDirectoryInstruction),
    Echo(EchoInstruction),
    // Spawn(SpawnInstruction),
}

pub struct LetInstruction {
    pub variable_name: VariableName,
    // pub variable_type: (),
    pub expression: Option<Expression>,
}
pub struct SetInstruction {
    pub variable_name: VariableName,
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
    variable_environment: &mut Environment,
) -> Result<(), InstructionExecutionError> {
    match instruction {
        Instruction::Let(LetInstruction {
            variable_name,
            expression: None,
        }) => {
            variable_environment
                .declare_typeless(variable_name)
                .unwrap();
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
                variable_environment
                    .declare_initialized(variable_name, variable_value)
                    .unwrap();
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
                match variable_environment.assign(&variable_name, variable_value) {
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
