use std::slice::Iter;
use crate::tokens::Token;

#[derive(Debug)]
pub struct Program(Function);
#[derive(Debug)]
pub struct Function(String, Statement);
#[derive(Debug)]
pub enum Statement {
    Return(Expression)
}
#[derive(Debug)]
pub enum Expression {
    Constant(i32)
}

pub fn parse(tokens: &mut Vec<Token>) -> Program {
    let mut iter = tokens.iter();
    let f = parse_function(&mut iter);
    let Token::EOF = iter.next().unwrap() else {
        panic!();
    };
    let None = iter.next() else { panic!() };
    return Program(f);
}

fn parse_function(tokens: &mut Iter<Token>) -> Function {
    let _return_type = match tokens.next().unwrap() {
        Token::KeywordInt => Token::KeywordInt,
        _ => panic!()
    };

    let identifier = match tokens.next().unwrap() {
        Token::Identifier(s) => s,
        _ => panic!()
    };

    let Token::OpenParenthesis = tokens.next().unwrap() else {
        panic!();
    };
    let Token::CloseParenthesis = tokens.next().unwrap() else {
        panic!();
    };

    let Token::OpenCurlyBrace = tokens.next().unwrap() else {
        panic!();
    };

    let s = parse_statement(tokens);

    let Token::CloseCurlyBrace = tokens.next().unwrap() else {
        panic!();
    };

    return Function(identifier.to_owned(), s);
}

fn parse_statement(tokens: &mut Iter<Token>) -> Statement {
    let Token::KeywordReturn = tokens.next().unwrap() else {
        panic!();
    };

    let e = parse_expression(tokens);

    let Token::Semicolon = tokens.next().unwrap() else {
        panic!();
    };
    return Statement::Return(e);
}

fn parse_expression(tokens: &mut Iter<Token>) -> Expression {
    let num = match tokens.next().unwrap() {
        Token::NumericConstant(s) => s,
        _ => panic!()
    };
    return Expression::Constant(num.parse().unwrap());
}