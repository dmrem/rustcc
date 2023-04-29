use crate::tokens::Token;
use std::slice::Iter;

#[derive(Debug)]
pub struct Program(pub Function);
#[derive(Debug)]
pub struct Function(pub String, pub Statement);
#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}
#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    UnaryOperation(Operation, Box<Expression>),
}
#[derive(Debug)]
pub enum Operation {
    Negation,
    BitwiseNot,
    LogicalNot,
}

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    let mut iter = tokens.iter();
    let f = parse_function(&mut iter);
    match iter.next().unwrap() {
        Token::EOF => (),
        token => panic!("Expected EOF, but found {:?}", token),
    };
    let None = iter.next() else { panic!("Found tokens after EOF!") };
    return Program(f);
}

fn parse_function(tokens: &mut Iter<Token>) -> Function {
    let _return_type = match tokens.next().unwrap() {
        Token::KeywordInt => Token::KeywordInt,
        token => panic!("Expected 'int', but found {:?}", token),
    };

    let identifier = match tokens.next().unwrap() {
        Token::Identifier(s) => s,
        token => panic!("Expected an identifier, but found {:?}", token),
    };

    match tokens.next().unwrap() {
        Token::OpenParenthesis => (),
        token => panic!("Expected '(', but found {:?}", token),
    };
    match tokens.next().unwrap() {
        Token::CloseParenthesis => (),
        token => panic!("Expected ')', but found {:?}", token),
    };

    match tokens.next().unwrap() {
        Token::OpenCurlyBrace => (),
        token => panic!("Expected '{{', but found {:?}", token),
    };

    let s = parse_statement(tokens);

    match tokens.next().unwrap() {
        Token::CloseCurlyBrace => (),
        token => panic!("Expected '}}', but found {:?}", token),
    };

    return Function(identifier.to_owned(), s);
}

fn parse_statement(tokens: &mut Iter<Token>) -> Statement {
    match tokens.next().unwrap() {
        Token::KeywordReturn => (),
        token => panic!("Expected 'return', but found {:?}", token),
    };

    let e = parse_expression(tokens);

    match tokens.next().unwrap() {
        Token::Semicolon => (),
        token => panic!("Expected ';', but found {:?}", token),
    };
    return Statement::Return(e);
}

fn parse_expression(tokens: &mut Iter<Token>) -> Expression {
    return match tokens.next().unwrap() {
        Token::NumericConstant(s) => Expression::Constant(
            s.parse()
                .unwrap_or_else(|_| panic!("Failed to parse '{}' as a number", s)),
        ),
        Token::Negation => {
            Expression::UnaryOperation(Operation::Negation, Box::from(parse_expression(tokens)))
        }
        Token::BitwiseNot => {
            Expression::UnaryOperation(Operation::BitwiseNot, Box::from(parse_expression(tokens)))
        }
        Token::LogicalNot => {
            Expression::UnaryOperation(Operation::LogicalNot, Box::from(parse_expression(tokens)))
        }
        token => panic!(
            "Expected a numeric constant or expression, but found {:?}",
            token
        ),
    };
}
