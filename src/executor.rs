pub mod variables;

pub mod instruction;

pub mod expression;

pub mod operation;

#[test]
fn test_execution() {
    use expression::Expression;
    use instruction::{
        EchoInstruction, Instruction, LetInstruction, SetInstruction, execute_instruction,
    };
    use operation::Operation;
    use variables::{Environment, VariableValue};
    let mut ve = Environment::new_with_default_globals();
    ve.push_scope();
    let instructions = [
        Instruction::Let(LetInstruction {
            variable_name: "x".to_string(),
            variable_kind: None,
            expression: Some(Expression::Value(VariableValue::LiteralInteger(12))),
        }),
        Instruction::Echo(EchoInstruction {
            expressions: vec![Expression::Variable("x".to_string())],
        }),
        Instruction::Set(SetInstruction {
            variable_name: "x".to_string(),
            expression: Expression::Operation(Box::new(Operation::Add {
                lhs: Expression::Variable("x".to_string()),
                rhs: Expression::Value(VariableValue::LiteralInteger(1)),
            })),
        }),
        Instruction::Echo(EchoInstruction {
            expressions: vec![Expression::Variable("x".to_string())],
        }),
        Instruction::Set(SetInstruction {
            variable_name: "x".to_string(),
            expression: Expression::Operation(Box::new(Operation::Multiply {
                lhs: Expression::Variable("x".to_string()),
                rhs: Expression::Value(VariableValue::LiteralInteger(3)),
            })),
        }),
        Instruction::Echo(EchoInstruction {
            expressions: vec![Expression::Variable("x".to_string())],
        }),
    ];
    for i in instructions {
        execute_instruction(i, &mut ve).unwrap();
    }
}
