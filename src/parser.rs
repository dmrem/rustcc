use crate::parser::BinaryOperation::{Add, Divide, Multiply, Subtract};
use crate::parser::Expression::{BinaryOpExpression, Constant, UnaryOpExpression};
use crate::parser::UnaryOperation::{BitwiseNot, LogicalNot, Negation};
use crate::tokens::Token;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct Program(pub Function);

#[derive(Debug)]
pub struct Function(pub &'static str, pub Statement);

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    UnaryOpExpression(UnaryOperation, Box<Expression>),
    BinaryOpExpression(BinaryOperation, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOperation {
    Negation,
    BitwiseNot,
    LogicalNot,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn parse(tokens: Vec<Token>) -> Program {
    let mut iter = tokens.iter().peekable();
    let f = parse_function(&mut iter);
    match iter.next().unwrap() {
        Token::EOF => (),
        token => panic!("Expected EOF, but found {:?}", token),
    };
    if let Some(token) = iter.next() { panic!("Found token after EOF: {:?}", token) };

    Program(f)
}

fn parse_function(tokens: &mut Peekable<Iter<Token>>) -> Function {
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

    Function(identifier.to_owned(), s)
}

fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> Statement {
    match tokens.next().unwrap() {
        Token::KeywordReturn => (),
        token => panic!("Expected 'return', but found {:?}", token),
    };

    let e = parse_expression(tokens);

    match tokens.next().unwrap() {
        Token::Semicolon => (),
        token => panic!("Expected ';', but found {:?}", token),
    };
    Statement::Return(e)
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    let mut term = parse_term(tokens);
    while let Some(Token::Plus) | Some(Token::Minus) = tokens.peek() {
        let op = match tokens.next() {
            Some(Token::Plus) => Add,
            Some(Token::Minus) => Subtract,
            _ => panic!("This will never happen"),
        };
        let next_term = parse_term(tokens);
        term = BinaryOpExpression(op, Box::from(term), Box::from(next_term));
    }
    term
}

fn parse_term(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    let mut factor = parse_factor(tokens);
    while let Some(Token::Asterisk) | Some(Token::ForwardSlash) = tokens.peek() {
        let op = match tokens.next() {
            Some(Token::Asterisk) => Multiply,
            Some(Token::ForwardSlash) => Divide,
            _ => panic!("This will never happen"),
        };
        let next_factor = parse_factor(tokens);
        factor = BinaryOpExpression(op, Box::from(factor), Box::from(next_factor));
    }
    factor
}

fn parse_factor(tokens: &mut Peekable<Iter<Token>>) -> Expression {
    match tokens.next().unwrap() {
        Token::OpenParenthesis => {
            let expression = parse_expression(tokens);
            match tokens.next().unwrap() {
                Token::CloseParenthesis => expression,
                token => panic!("Expected ')', but found {:?}", token),
            }
        }
        Token::Minus => UnaryOpExpression(Negation, Box::from(parse_factor(tokens))),
        Token::Tilde => UnaryOpExpression(BitwiseNot, Box::from(parse_factor(tokens))),
        Token::ExclamationPoint => UnaryOpExpression(LogicalNot, Box::from(parse_factor(tokens))),

        Token::NumericConstant(s) => Constant(s.parse::<i32>().unwrap()),
        t => panic!("Could not parse expression! Next token: {:?}", t),
    }
}
