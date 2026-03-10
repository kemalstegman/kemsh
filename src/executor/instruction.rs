use crate::executor::{
    expression::{Expression, execute_expression},
    variables::{Environment, VariableKind, VariableName, VariableValue},
};

// let var = [expr];
// let var;
// let var: [type];
// let var: [type] = [expr];
// var = [expr];
// echo [expr];
// cd [expr: string];
// if [expr: boolean] {}
// loop {}
// while [expr: boolean] {}
// ??for??
// ??return [?expr];??
// ??break [?expr];??
// ??fn??

#[allow(dead_code)]
pub enum Instruction {
    Let(LetInstruction),
    Set(SetInstruction),
    ChangeDirectory(ChangeDirectoryInstruction),
    Echo(EchoInstruction),
}

pub struct LetInstruction {
    pub variable_name: VariableName,
    pub variable_kind: Option<VariableKind>,
    pub expression: Option<Expression>,
}
pub struct SetInstruction {
    pub variable_name: VariableName,
    pub expression: Expression,
}
pub struct ChangeDirectoryInstruction {
    pub expression: Expression,
}
pub struct EchoInstruction {
    pub expressions: Vec<Expression>,
}

#[allow(dead_code)]
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
            variable_kind: None,
            expression: None,
        }) => match variable_environment.declare_typeless(variable_name) {
            Ok(()) => (),
            Err(err) => {
                return Err(InstructionExecutionError {
                    message: format!("variable environment error: {err:?}"),
                });
            }
        },
        Instruction::Let(LetInstruction {
            variable_name,
            variable_kind: Some(variable_kind),
            expression: None,
        }) => match variable_environment.declare_typed(variable_name, variable_kind) {
            Ok(()) => (),
            Err(err) => {
                return Err(InstructionExecutionError {
                    message: format!("variable environment error: {err:?}"),
                });
            }
        },
        Instruction::Let(LetInstruction {
            variable_name,
            variable_kind: None,
            expression: Some(expression),
        }) => match execute_expression(expression, variable_environment) {
            Ok(Some(variable_value)) => {
                match variable_environment.declare_initialized(variable_name, variable_value) {
                    Ok(()) => (),
                    Err(err) => {
                        return Err(InstructionExecutionError {
                            message: format!("variable environment error: {err:?}"),
                        });
                    }
                }
            }
            Ok(None) => {
                return Err(InstructionExecutionError {
                    message: format!("expected value from expression"),
                });
            }
            Err(err) => {
                return Err(InstructionExecutionError {
                    message: format!("{err:?}"),
                });
            }
        },
        Instruction::Let(LetInstruction {
            variable_name,
            variable_kind: Some(variable_kind),
            expression: Some(expression),
        }) => match execute_expression(expression, variable_environment) {
            Ok(Some(variable_value)) => {
                if variable_value.kind() == variable_kind {
                    match variable_environment.declare_initialized(variable_name, variable_value) {
                        Ok(()) => (),
                        Err(err) => {
                            return Err(InstructionExecutionError {
                                message: format!("variable environment error: {err:?}"),
                            });
                        }
                    }
                } else {
                    return Err(InstructionExecutionError {
                        message: format!(
                            "expected {:?} from expression, got {:?}",
                            variable_kind,
                            variable_value.kind()
                        ),
                    });
                }
            }
            Ok(None) => {
                return Err(InstructionExecutionError {
                    message: format!("expected value from expression"),
                });
            }
            Err(err) => {
                return Err(InstructionExecutionError {
                    message: format!("{err:?}"),
                });
            }
        },
        Instruction::Set(SetInstruction {
            variable_name,
            expression,
        }) => match execute_expression(expression, variable_environment) {
            Ok(Some(variable_value)) => {
                match variable_environment.assign(&variable_name, variable_value) {
                    Ok(()) => (),
                    Err(err) => {
                        return Err(InstructionExecutionError {
                            message: format!("variable environment error: {err:?}"),
                        });
                    }
                }
            }
            Ok(None) => {
                return Err(InstructionExecutionError {
                    message: format!("expected value from expression"),
                });
            }
            Err(err) => {
                return Err(InstructionExecutionError {
                    message: format!("{err:?}"),
                });
            }
        },
        Instruction::ChangeDirectory(ChangeDirectoryInstruction { expression }) => {
            match execute_expression(expression, variable_environment) {
                Ok(Some(VariableValue::LiteralString(string_path))) => {
                    match std::env::set_current_dir(std::path::Path::new(&string_path)) {
                        Ok(()) => (),
                        Err(err) => {
                            return Err(InstructionExecutionError {
                                message: format!("error changing directory: {err:?}"),
                            });
                        }
                    }
                }
                Ok(Some(value)) => {
                    return Err(InstructionExecutionError {
                        message: format!("expected string from expression, got {:?}", value.kind()),
                    });
                }
                Ok(None) => {
                    return Err(InstructionExecutionError {
                        message: format!("expected value from expression"),
                    });
                }
                Err(err) => {
                    return Err(InstructionExecutionError {
                        message: format!("{err:?}"),
                    });
                }
            }
        }
        Instruction::Echo(EchoInstruction { expressions }) => {
            match expressions
                .into_iter()
                .map(
                    |expression| match execute_expression(expression, variable_environment) {
                        Ok(Some(value)) => Ok(value),
                        Ok(None) => Err(InstructionExecutionError {
                            message: format!("expected value from expression"),
                        }),
                        Err(err) => Err(InstructionExecutionError {
                            message: format!("{err:?}"),
                        }),
                    },
                )
                .collect::<Result<Vec<VariableValue>, InstructionExecutionError>>()
            {
                Ok(values) => {
                    for (i, value) in values.into_iter().enumerate() {
                        if i != 0 {
                            print!(" ");
                        }
                        print!("{value:?}");
                    }
                    print!("\n");
                }
                Err(err) => return Err(err),
            }
        }
    }
    Ok(())
}
