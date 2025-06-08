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
    let code = "let a = add 1 2 in let b = a in 3";
    dbg!(Expr::parse(tokenize(code)?));
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
