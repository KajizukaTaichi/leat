mod eval;
mod lexer;
mod parse;

use indexmap::IndexMap;
use rustyline::{DefaultEditor, error::ReadlineError};
pub use {eval::*, lexer::*};

fn main() {
    println!("Hello, world!");
    let mut rl = DefaultEditor::new().unwrap();
    let mut buf = String::new();
    let mut env = stdlib();
    loop {
        match rl.readline("> ") {
            Ok(code) => {
                buf.push_str(&code);
                if let Some(Some(ast)) = lex(&buf).map(|x| Expr::parse(x)) {
                    println!("{:?}", ast.eval(&mut env));
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
    Call(Box<Expr>, Box<Expr>),
    Variable(String),
    Literal(Value),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Lambda(Lambda),
    Type(Type),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Number,
    String,
    Bool,
    Lambda,
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
    UndefinedName(String),
    CallNotLambda(Expr),
    InvalidBind(Expr),
    InvalidArg(Expr),
    InvalidOperation,
}

fn stdlib() -> Env {
    macro_rules! curry_2arg {
        ($processing: expr) => {
            Value::Lambda(Lambda::BuiltIn(
                |a, mut env| {
                    Ok(Value::Lambda(Lambda::BuiltIn(
                        |b, env| {
                            let a = env.get("a").unwrap().clone();
                            $processing(a, b)
                        },
                        {
                            env.insert(String::from("a"), a);
                            env
                        },
                    )))
                },
                IndexMap::new(),
            ))
        };
    }
    IndexMap::from([
        (
            String::from("+"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a + b)),
                    [Value::String(a), Value::String(b)] => Ok(Value::String(a + &b)),
                    _ => Err(LeatError::InvalidOperation),
                }
            }),
        ),
        (
            String::from("-"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a - b)),
                    _ => Err(LeatError::InvalidOperation),
                }
            }),
        ),
        (
            String::from("*"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a * b)),
                    [Value::String(a), Value::Number(b)] => Ok(Value::String(a.repeat(b as usize))),
                    _ => Err(LeatError::InvalidOperation),
                }
            }),
        ),
        (
            String::from("/"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a / b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("=="),
            curry_2arg!(|a, b| Ok(Value::Bool(a == b))),
        ),
        (
            String::from(">"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a > b)),
                    [Value::String(a), Value::String(b)] => Ok(Value::Bool(a > b)),
                    _ => Err(LeatError::InvalidOperation),
                }
            }),
        ),
        (
            String::from("<"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a < b)),
                [Value::String(a), Value::String(b)] => Ok(Value::Bool(a < b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("&"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Bool(a), Value::Bool(b)] => Ok(Value::Bool(a & b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("|"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Bool(a), Value::Bool(b)] => Ok(Value::Bool(a | b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("cast"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Type(Type::String)] => {
                        Ok(Value::String(a.to_string()))
                    }
                    [Value::String(a), Value::Type(Type::Number)] => {
                        if let Ok(n) = a.parse::<f64>() {
                            Ok(Value::Number(n))
                        } else {
                            Err(LeatError::InvalidOperation)
                        }
                    }
                    _ => Err(LeatError::InvalidOperation),
                }
            }),
        ),
        (
            String::from("typeof"),
            Value::Lambda(Lambda::BuiltIn(
                |a, _| match a {
                    Value::Number(_) => Ok(Value::Type(Type::Number)),
                    Value::String(_) => Ok(Value::Type(Type::String)),
                    Value::Bool(_) => Ok(Value::Type(Type::Bool)),
                    Value::Lambda(_) => Ok(Value::Type(Type::Lambda)),
                    Value::Type(_) => Ok(Value::Type(Type::Kind)),
                },
                IndexMap::new(),
            )),
        ),
    ])
}
