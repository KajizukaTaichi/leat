use crate::*;
use std::fmt::{Display, Formatter, Result};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expr::Literal(value) => write!(f, "{value}"),
            Expr::Call(func, arg) => write!(f, "({func} {arg})"),
            Expr::Array(array) => {
                let array = array
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{array}]",)
            }
            Expr::Let(name, value, expr) => write!(f, "(let {name} := {value} in {expr})"),
            Expr::If(cond, then, els) => write!(f, "(if {cond} then {then} else {els})"),
            Expr::Try(risky, callback) => write!(f, "(try {risky} catch {callback})"),
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
            Value::Array(array) => {
                let array = array
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{array}]",)
            }
            Value::Lambda(Lambda::UserDefined(arg, body, env)) => {
                write!(f, "(\\{arg}. {})", {
                    let mut body = body.clone();
                    for (key, val) in env {
                        if !matches!(val, Value::Lambda(Lambda::BuiltIn(_, _))) {
                            let key = &Expr::Variable(key.to_owned());
                            let val = &Expr::Literal(val.clone());
                            *body = body.replace(key, val);
                        }
                    }
                    body
                })
            }
            Value::Lambda(Lambda::BuiltIn(func, _)) => write!(f, "(\\x. {func:?})"),
            Value::Type(typ) => write!(f, "{typ}"),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Type::Number => write!(f, "#number"),
            Type::String => write!(f, "#string"),
            Type::Bool => write!(f, "#bool"),
            Type::Array => write!(f, "#array"),
            Type::Lambda => write!(f, "#lambda"),
            Type::Kind => write!(f, "#kind"),
        }
    }
}
impl Display for LeatError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LeatError::Undefined(name) => write!(f, "can't refer undefined variable `{name}`"),
            LeatError::CantReassign(name) => write!(f, "can't reassign variable `{name}` again"),
            LeatError::TypeMismatch(name) => write!(f, "type mismatch, so expected `{name}`"),
            LeatError::NonLambda(expr) => write!(f, "can't apply non-lambda value `{expr}`"),
            LeatError::InvalidArg(name) => write!(f, "invalid argument's name `{name}` for lambda"),
            LeatError::InvalidBind(expr) => write!(f, "invalid bind `{expr}` using let expression"),
            LeatError::InvalidOperation => write!(f, "invalid operation, not allowed in Leat"),
            LeatError::UserDefined(msg) => write!(f, "{msg}"),
        }
    }
}
