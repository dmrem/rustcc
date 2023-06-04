use crate::code_generator::AsmToken::{
    FunctionStart, InstructionNoArgs, InstructionOneArg, InstructionTwoArgs,
};
use crate::parser::{Expression, Function, Program, Statement, UnaryOperation};

enum AsmToken {
    FunctionStart(&'static str),
    InstructionNoArgs(&'static str),
    InstructionOneArg(&'static str, &'static str),
    InstructionTwoArgs(&'static str, &'static str, &'static str),
}

struct AsmProgram(pub Vec<AsmToken>);

impl From<&AsmToken> for String {
    fn from(value: &AsmToken) -> Self {
        match value {
            FunctionStart(name) => format!(".globl {}\n{}:", name, name),
            InstructionNoArgs(instruction) => format!("  {}", instruction),
            InstructionOneArg(instruction, arg) => format!("  {} {}", instruction, arg),
            InstructionTwoArgs(instruction, arg1, arg2) => {
                format!("  {} {}, {}", instruction, arg1, arg2)
            }
        }
    }
}

impl From<AsmProgram> for String {
    fn from(value: AsmProgram) -> Self {
        value
            .0
            .iter()
            .map(|token| token.into())
            .collect::<Vec<String>>()
            .join("\n")
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
            x.push(InstructionNoArgs("ret"));
            x
        }
    }
}

fn generate_asm_for_expression(expression: Expression) -> Vec<AsmToken> {
    match expression {
        Expression::Constant(int) => vec![InstructionTwoArgs(
            "movl",
            int.to_string().as_str(),
            "%eax",
        )],
        Expression::UnaryOpExpression(op, exp) => match op {
            UnaryOperation::Negation => {
                let mut x = generate_asm_for_expression(*exp);
                x.extend(vec![InstructionOneArg("neg", "%eax")]);
                x
            }
            UnaryOperation::BitwiseNot => {
                let mut x = generate_asm_for_expression(*exp);
                x.extend(vec![InstructionOneArg("not", "%eax")]);
                x
            }
            UnaryOperation::LogicalNot => {
                let mut x = generate_asm_for_expression(*exp);
                x.extend(vec![
                    InstructionTwoArgs("cmpl", "$0", "%eax"),
                    InstructionTwoArgs("movl", "$0", "%eax"),
                    InstructionOneArg("sete", "%al"),
                ]);
                x
            }
        },
        Expression::BinaryOpExpression(op, exp1, exp2) => {
            panic!()
        }
    }
}
