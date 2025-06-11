mod eval;
mod fmt;
mod lexer;
mod meta;
mod parse;
mod repl;
mod stdlib;
mod token;

use indexmap::IndexMap;
pub use {lexer::lex, repl::repl, stdlib::stdlib, token::Token};

fn main() {
    repl();
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
    Null,
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

#[derive(Clone, Debug)]
pub enum Lambda {
    BuiltIn(fn(Value, Env) -> Result<Value, LeatError>, Env),
    UserDefined(String, Box<Expr>, Env),
}

impl PartialEq for Lambda {
    fn eq(&self, other: &Lambda) -> bool {
        match [self, other] {
            [Lambda::BuiltIn(a, _), Lambda::BuiltIn(b, _)] => format!("{a:?}") == format!("{b:?}"),
            [Lambda::UserDefined(_, a, _), Lambda::UserDefined(_, b, _)] => a == b,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LeatError {
    Undefined(String),
    CantReassign(String),
    NonLambda(Expr),
    InvalidBind(Expr),
    InvalidArg(Expr),
    TypeMismatch(Type),
    InvalidOperation,
    UserDefined(String),
}
