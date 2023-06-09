#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    KeywordInt,
    KeywordReturn,
    Identifier(String),
    NumericConstant(String),
    OpenParenthesis,
    CloseParenthesis,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Semicolon,
    Minus,
    Tilde,
    ExclamationPoint,
    Plus,
    Asterisk,
    ForwardSlash,
    EOF,
}
