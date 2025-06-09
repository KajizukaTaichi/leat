use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expr::Literal(value) => write!(f, "{value}"),
            Expr::Call(func, arg) => write!(f, "({func} {arg})"),
            Expr::Let(name, value, expr) => write!(f, "(let {name} := {value} in {expr})"),
            Expr::If(cond, then, els) => write!(f, "(if {cond} then {then} else {els})"),
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
            Value::Lambda(Lambda::UserDefined(arg, body, _)) => write!(f, "(\\{arg}. {body})"),
            Value::Lambda(Lambda::BuiltIn(func, _)) => write!(f, "(\\x. {func:?})"),
            Value::Type(typ) => write!(f, "{}", format!("#{typ:?}").to_lowercase()),
        }
    }
}

impl Display for LeatError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LeatError::Undefined(name) => write!(f, "can't refer undefined variable name `{name}`"),
            LeatError::NonLambda(expr) => write!(f, "can't apply non-lambda value `{expr}`"),
            LeatError::InvalidArg(name) => write!(f, "invalid argument's name `{name}` for lambda"),
            LeatError::InvalidBind(expr) => write!(f, "invalid bind `{expr}` using let expression"),
            LeatError::InvalidOperation => write!(f, "invalid operation, not allowed in Leat"),
        }
    }
}
