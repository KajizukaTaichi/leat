mod eval;
mod lexer;
mod parse;

use indexmap::IndexMap;
pub use lexer::*;

fn main() {
    println!("Hello, world!");
    run().unwrap();
}

fn run() -> Option<()> {
    let code = "let b = + 1 3 in a";
    let ast = Expr::parse(tokenize(code)?)?;
    let env = &mut IndexMap::from([(
        String::from("+"),
        Value::Lambda(Lambda::BuiltIn(
            |a, mut env| {
                Some(Value::Lambda(Lambda::BuiltIn(
                    |b, env| {
                        let a = env.get("a")?.clone();
                        match [a, b] {
                            [Value::Number(a), Value::Number(b)] => Some(Value::Number(a + b)),
                            [Value::String(a), Value::String(b)] => Some(Value::String(a + &b)),
                            _ => None,
                        }
                    },
                    {
                        env.insert(String::from("a"), a);
                        env
                    },
                )))
            },
            IndexMap::new(),
        )),
    )]);
    dbg!(ast.eval(env));
    Some(())
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
