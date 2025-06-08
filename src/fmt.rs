use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expr::Literal(value) => write!(f, "{value}"),
            Expr::Call(func, arg) => write!(f, "({func} {arg})"),
            Expr::Let(name, value, expr) => write!(f, "let {name} := {value} in {expr}"),
            Expr::If(cond, then, els) => write!(f, "if {cond} then {then} else {els}"),
            Expr::Variable(name) => write!(f, "{name}"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "\"{s}\""),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Lambda(Lambda::UserDefined(arg, body, _)) => write!(f, "\\{arg}. {body}"),
            Value::Lambda(Lambda::BuiltIn(func, _)) => write!(f, "\\x.{func:?}"),
            Value::Type(typ) => write!(f, "{}", format!("{typ:?}").to_lowercase()),
        }
    }
}
