use crate::{lexer::f_string, *};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    FString(Vec<Token>),
    Bool(bool),
    Ident(String),
    Nest(Vec<Token>),
    Array(Vec<Token>),
    Type(Type),
    Lambda,
    Dot,
    Comma,
    Let,
    Assign,
    In,
    If,
    Then,
    Else,
    Try,
    Catch,
}

impl Token {
    pub fn new(token: String) -> Option<Token> {
        Some(if token == "let" {
            Token::Let
        } else if token == ":=" {
            Token::Assign
        } else if token == "in" {
            Token::In
        } else if token == "try" {
            Token::Try
        } else if token == "catch" {
            Token::Catch
        } else if token == "if" {
            Token::If
        } else if token == "then" {
            Token::Then
        } else if token == "else" {
            Token::Else
        } else if token == "#number" {
            Token::Type(Type::Number)
        } else if token == "#string" {
            Token::Type(Type::String)
        } else if token == "#bool" {
            Token::Type(Type::Bool)
        } else if token == "#array" {
            Token::Type(Type::Array)
        } else if token == "#lambda" {
            Token::Type(Type::Lambda)
        } else if token == "#kind" {
            Token::Type(Type::Kind)
        } else if token == "\\" {
            Token::Lambda
        } else if token == "." {
            Token::Dot
        } else if token == "," {
            Token::Comma
        } else if let Ok(b) = token.parse::<bool>() {
            Token::Bool(b)
        } else if let Ok(n) = token.parse::<f64>() {
            Token::Number(n)
        } else if let Some(Some(string)) = token.strip_prefix("\"").map(|x| x.strip_suffix("\"")) {
            Token::String(string.to_string())
        } else if let Some(Some(string)) = token.strip_prefix("f\"").map(|x| x.strip_suffix("\"")) {
            Token::FString(f_string(string)?)
        } else if let Some(Some(nest)) = token.strip_prefix("(").map(|x| x.strip_suffix(")")) {
            Token::Nest(lex(nest)?)
        } else if let Some(Some(nest)) = token.strip_prefix("[").map(|x| x.strip_suffix("]")) {
            Token::Array(lex(nest)?)
        } else {
            Token::Ident(token)
        })
    }
}
