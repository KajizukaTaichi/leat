mod eval;
mod lexer;
mod parse;

use indexmap::IndexMap;
pub use lexer::*;
use rustyline::{DefaultEditor, error::ReadlineError};

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
                    if let Some(result) = ast.eval(&mut env) {
                        println!("{result:?}");
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

fn stdlib() -> Env {
    macro_rules! curry_2arg {
        ($processing: expr) => {
            Value::Lambda(Lambda::BuiltIn(
                |a, mut env| {
                    Some(Value::Lambda(Lambda::BuiltIn(
                        |b, env| {
                            let a = env.get("a")?.clone();
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
                    [Value::Number(a), Value::Number(b)] => Some(Value::Number(a + b)),
                    [Value::String(a), Value::String(b)] => Some(Value::String(a + &b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("-"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Some(Value::Number(a - b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("*"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Some(Value::Number(a * b)),
                    [Value::String(a), Value::Number(b)] => {
                        Some(Value::String(a.repeat(b as usize)))
                    }
                    _ => None,
                }
            }),
        ),
        (
            String::from("/"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Some(Value::Number(a / b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("=="),
            curry_2arg!(|a, b| { Some(Value::Bool(a == b)) }),
        ),
        (
            String::from(">"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Some(Value::Bool(a > b)),
                    [Value::String(a), Value::String(b)] => Some(Value::Bool(a > b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("<"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Number(b)] => Some(Value::Bool(a < b)),
                    [Value::String(a), Value::String(b)] => Some(Value::Bool(a < b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("&"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Bool(a), Value::Bool(b)] => Some(Value::Bool(a & b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("|"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Bool(a), Value::Bool(b)] => Some(Value::Bool(a | b)),
                    _ => None,
                }
            }),
        ),
        (
            String::from("cast"),
            curry_2arg!(|a, b| {
                match [a, b] {
                    [Value::Number(a), Value::Type(Type::String)] => {
                        Some(Value::String(a.to_string()))
                    }
                    [Value::String(a), Value::Type(Type::Number)] => {
                        if let Ok(n) = a.parse::<f64>() {
                            Some(Value::Number(n))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }),
        ),
        (
            String::from("typeof"),
            Value::Lambda(Lambda::BuiltIn(
                |a, _| match a {
                    Value::Number(_) => Some(Value::Type(Type::Number)),
                    Value::String(_) => Some(Value::Type(Type::String)),
                    Value::Bool(_) => Some(Value::Type(Type::Bool)),
                    Value::Lambda(_) => Some(Value::Type(Type::Lambda)),
                    Value::Type(_) => Some(Value::Type(Type::Kind)),
                },
                IndexMap::new(),
            )),
        ),
    ])
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
    BuiltIn(fn(Value, Env) -> Option<Value>, Env),
    UserDefined(String, Box<Expr>, Env),
}
