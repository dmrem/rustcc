use crate::parser::{Expression, Function, Program, Statement};

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
                    " movl ${}, %eax\n",
                    " ret"
                },
                generate_asm_for_expression(expression)
            )
        }
    }
}

fn generate_asm_for_expression(expression: Expression) -> String {
    match expression {
        Expression::Constant(int) => int.to_string()
    }
}