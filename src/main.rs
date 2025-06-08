mod eval;

use indexmap::IndexMap;

fn main() {
    println!("Hello, world!");
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
    BuiltIn(fn(Value) -> Option<Value>, Env),
    UserDefined(String, Box<Expr>, Env),
}
