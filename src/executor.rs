// use std::collections::HashMap;

// use crate::unit_instructions::{
//     AssignCommand, BraceBlock, Expression, LetCommand, Literal, UnitInstruction,
// };

// pub struct VariableEnvironment {
//     scopes: Vec<ScopedVariableEnvironment>,
// }

// pub struct ScopedVariableEnvironment {
//     variables: HashMap<String, Literal>,
// }

// pub fn execute(
//     environment: &mut VariableEnvironment,
//     instructions: impl Iterator<Item = UnitInstruction>,
// ) -> Result<(), ()> {
//     for instruction in instructions {
//         match instruction {
//             UnitInstruction::Let(LetCommand { variable, value }) => {
//                 match execute_expression(environment, value) {
//                     Ok(l) => {
//                         environment
//                             .scopes
//                             .last_mut()
//                             .unwrap()
//                             .variables
//                             .insert(variable, l);
//                     }
//                     Err(_) => return Err(()),
//                 }
//             }
//             UnitInstruction::Assign(AssignCommand { variable, value }) => {
//                 match execute_expression(environment, value) {
//                     Ok(l) => match environment
//                         .scopes
//                         .last_mut()
//                         .unwrap()
//                         .variables
//                         .get_mut(&variable)
//                     {
//                         None => return Err(()),
//                         Some(variable) => *variable = l,
//                     },

//                     Err(_) => return Err(()),
//                 }
//             }
//             UnitInstruction::For => return Err(()),
//             UnitInstruction::While => return Err(()),
//             UnitInstruction::Loop => return Err(()),
//             UnitInstruction::Return => return Err(()),
//             UnitInstruction::Break => return Err(()),
//             UnitInstruction::Run => return Err(()),
//             UnitInstruction::Spawn => return Err(()),
//             UnitInstruction::CD => return Err(()),
//         }
//     }
//     Ok(())
// }

// fn execute_expression(
//     environment: &mut VariableEnvironment,
//     expression: Expression,
// ) -> Result<Literal, ()> {
//     match expression {
//         Expression::Literal(l) => Ok(l),
//         Expression::Variable(v) => match environment.scopes.last().unwrap().variables.get(&v) {
//             None => return Err(()),
//             Some(&l) => Ok(l),
//         },
//         Expression::Operation(op) => return Err(()),
//         Expression::Block(block) => execute_block(environment, block),
//     }
// }

// fn execute_block(environment: &VariableEnvironment, block: BraceBlock) -> Result<Literal, ()> {
//     Err(())
// }

use std::collections::HashMap;

pub struct VariableEnvironment {
    scopes: Vec<HashMap<String, Option<VariableValue>>>,
}

impl VariableEnvironment {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) -> Option<()> {
        match self.scopes.pop() {
            Some(_) => Some(()),
            None => None,
        }
    }
    pub fn declare_variable(&mut self, variable_name: String) {
        if self
            .scopes
            .last_mut()
            .unwrap()
            .insert(variable_name, None)
            .is_some()
        {
            panic!()
        }
    }
    pub fn declare_and_initialize_variable(
        &mut self,
        variable_name: String,
        variable_value: VariableValue,
    ) {
        if self
            .scopes
            .last_mut()
            .unwrap()
            .insert(variable_name, Some(variable_value))
            .is_some()
        {
            panic!()
        }
    }
    pub fn assign_variable(
        &mut self,
        variable_name: String,
        variable_value: VariableValue,
    ) -> Result<(), ()> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value) = scope.get_mut(&variable_name) {
                *value = Some(variable_value);
                return Ok(());
            }
        }
        Err(())
    }
    pub fn get_variable(&self, variable_name: String) -> Result<Option<VariableValue>, ()> {
        for scope in self.scopes.iter().rev() {
            if let Some(possible_value) = scope.get(&variable_name) {
                match possible_value {
                    Some(value) => return Ok(Some(value.clone())),
                    None => return Err(()),
                }
            }
        }
        Ok(None)
    }
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
    variable_environment: &mut VariableEnvironment,
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
                match variable_environment.assign_variable(variable_name, variable_value) {
                    Err(()) => Err(InstructionExecutionError {
                        message: "Variable dne".to_string(),
                    })?,
                    Ok(()) => (),
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
    variable_environment: &mut VariableEnvironment,
) -> Result<Option<VariableValue>, ExpressionExecutionError> {
    match expression {
        Expression::Value(value) => Ok(Some(value)),
        Expression::Variable(variable_name) => {
            match variable_environment.get_variable(variable_name) {
                Ok(None) => Err(ExpressionExecutionError {
                    message: "Variable dne".to_string(),
                }),
                Err(()) => Err(ExpressionExecutionError {
                    message: "Variable uninitialized".to_string(),
                }),
                Ok(Some(value)) => Ok(Some(value)),
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
    variable_environment: &mut VariableEnvironment,
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
