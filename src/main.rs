mod eval;
mod lexer;
mod parse;

use indexmap::IndexMap;
pub use lexer::*;

fn main() {
    println!("Hello, world!");
    dbg!(run(r#"let inc n = + n 1 in inc (inc 5)"#));
}

fn run(code: &str) -> Option<Value> {
    let ast = Expr::parse(tokenize(code)?)?;
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

    let env = &mut IndexMap::from([
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
                    [Value::Number(a), Value::Number(b)] => Some(Value::Number(a - b)),
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
    ]);
    ast.eval(env)
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
}

type Env = IndexMap<String, Value>;

#[derive(Clone, Debug, PartialEq)]
pub enum Lambda {
    BuiltIn(fn(Value, Env) -> Option<Value>, Env),
    UserDefined(String, Box<Expr>, Env),
}
