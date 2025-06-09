mod eval;
mod fmt;
mod lexer;
mod meta;
mod parse;
mod stdlib;
mod token;

use indexmap::IndexMap;
use rustyline::{DefaultEditor, error::ReadlineError};
pub use {lexer::lex, stdlib::stdlib, token::Token};

fn main() {
    println!("Leat REPL");
    let mut rl = DefaultEditor::new().unwrap();
    let mut buf = String::new();
    let mut env = stdlib();
    loop {
        match rl.readline("> ") {
            Ok(code) => {
                buf.push_str(&code);
                buf.push_str("\n");
                rl.add_history_entry(code).unwrap();
                if let Some(Some(ast)) = lex(&buf).map(|x| Expr::parse(x)) {
                    match ast.eval(&mut env) {
                        Ok(res) => println!("{res}"),
                        Err(err) => println!("Error! {err}"),
                    }
                    buf.clear();
                }
            }
            Err(ReadlineError::Interrupted) => {
                buf.clear();
            }
            Err(ReadlineError::Eof) => break,
            _ => {}
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Box<Expr>, Box<Expr>, Box<Expr>),
    Try(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
    Variable(String),
    Literal(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Lambda(Lambda),
    Type(Type),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Number,
    String,
    Bool,
    Lambda,
    Array,
    Kind,
}

type Env = IndexMap<String, Value>;

#[derive(Clone, Debug, PartialEq)]
pub enum Lambda {
    BuiltIn(fn(Value, Env) -> Result<Value, LeatError>, Env),
    UserDefined(String, Box<Expr>, Env),
}

#[derive(Clone, Debug, PartialEq)]
pub enum LeatError {
    Undefined(String),
    CantReassign(String),
    NonLambda(Expr),
    InvalidBind(Expr),
    InvalidArg(Expr),
    InvalidOperation,
    UserDefined(String),
}
