use crate::code_generator::AsmToken::{InstructionNoArgs, InstructionOneArg, InstructionTwoArgs, FunctionStart};
use crate::parser::{Expression, Function, Operation, Program, Statement};

enum AsmToken {
    FunctionStart(String),
    InstructionNoArgs(String),
    InstructionOneArg(String, String),
    InstructionTwoArgs(String, String, String),
}

struct AsmProgram (pub Vec<AsmToken>);

impl From<&AsmToken> for String {
    fn from(value: &AsmToken) -> Self {
        match value {
            FunctionStart(name) => format!(".globl {}\n{}:", name, name),
            InstructionNoArgs(instruction) => format!("  {}", instruction),
            InstructionOneArg(instruction, arg) => format!("  {} {}", instruction, arg),
            InstructionTwoArgs(instruction, arg1, arg2) => format!("  {} {}, {}", instruction, arg1, arg2),
        }
    }
}

impl From<AsmProgram> for String {
    fn from(value: AsmProgram) -> Self {
        value.0.iter().map(|token| token.into()).collect::<Vec<String>>().join("\n")
    }
}

pub fn generate_code(program: Program) -> String {
    let Program(function): Program = program;
    AsmProgram(generate_asm_for_function(function)).into()
}

fn generate_asm_for_function(function: Function) -> Vec<AsmToken> {
    let Function(name, body) = function;

    let mut asm = vec![FunctionStart(name)];
    asm.extend(generate_asm_for_statement(body).into_iter());
    asm
}

fn generate_asm_for_statement(statement: Statement) -> Vec<AsmToken> {
    match statement {
        Statement::Return(expression) => {
            let mut x = vec![];
            x.extend(generate_asm_for_expression(expression));
            x.push(InstructionNoArgs("ret".into()));
            x
        }
    }
}

fn generate_asm_for_expression(expression: Expression) -> Vec<AsmToken> {
    match expression {
        Expression::Constant(int) => vec![InstructionTwoArgs("movl".into(), int.to_string(), "%eax".into())],
        Expression::UnaryOperation(op, exp) => match op {
            Operation::Negation => {
                let mut x = generate_asm_for_expression(*exp);
                x.extend(vec![InstructionOneArg("neg".into(), "%eax".into())]);
                x
            },
            Operation::BitwiseNot => {
                let mut x = generate_asm_for_expression(*exp);
                x.extend(vec![InstructionOneArg("not".into(), "%eax".into())]);
                x
            },
            Operation::LogicalNot => {
                let mut x = generate_asm_for_expression(*exp);
                x.extend(vec![
                    InstructionTwoArgs("cmpl".into(), "$0".into(), "%eax".into()),
                    InstructionTwoArgs("movl".into(), "$0".into(),"%eax".into()),
                    InstructionOneArg("sete".into(), "%al".into()),
                ]);
                x
            },
        },
    }
}
