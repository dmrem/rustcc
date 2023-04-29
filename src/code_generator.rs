use crate::parser::{Expression, Function, Operation, Program, Statement};
use std::fmt::format;

pub fn generate_code(program: Program) -> String {
    let Program(function): Program = program;
    generate_asm_for_function(function)
}

fn generate_asm_for_function(function: Function) -> String {
    let Function(name, body) = function;
    return format!(
        concat! {
        ".globl {}\n",
        "{}:\n",
        "{}\n"
        },
        name,
        name,
        generate_asm_for_statement(body)
    );
}

fn generate_asm_for_statement(statement: Statement) -> String {
    match statement {
        Statement::Return(expression) => {
            format!(
                concat! {
                "{}",
                " ret"
                },
                generate_asm_for_expression(expression)
            )
        }
    }
}

fn generate_asm_for_expression(expression: Expression) -> String {
    match expression {
        Expression::Constant(int) => format!(" movl ${}, %eax\n", int.to_string()),
        Expression::UnaryOperation(op, exp) => match op {
            Operation::Negation => format!(
                concat!("{}", " neg %eax\n",),
                generate_asm_for_expression(*exp)
            ),
            Operation::BitwiseNot => format!(
                concat!("{}", " not %eax\n",),
                generate_asm_for_expression(*exp)
            ),
            Operation::LogicalNot => format!(
                concat!("{}", " cmpl $0, %eax\n", " movl $0, %eax\n", " sete %al\n",),
                generate_asm_for_expression(*exp)
            ),
        },
    }
}
