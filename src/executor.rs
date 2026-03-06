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
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    pub fn pop_scope(&mut self) -> Option<()> {
        match self.scopes.pop() {
            Some(_) => Some(()),
            None => None,
        }
    }
    pub fn declare_variable(&mut self) {
        todo!()
    }
    pub fn assign_variable(&mut self) {
        todo!()
    }
}

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
    // ChangeDirectory(ChangeDirectoryInstruction),
    Echo(EchoInstruction),
}

pub struct LetInstruction;
// pub struct ChangeDirectoryInstruction;
pub struct EchoInstruction;

// todo!() variants of process execution
// pub struct SpawnInstruction;
