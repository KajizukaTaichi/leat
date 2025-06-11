use crate::*;

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match [self, other] {
            [Value::Number(a), Value::Number(b)] => a == b,
            [Value::String(a), Value::String(b)] => a == b,
            [Value::Bool(a), Value::Bool(b)] => a == b,
            [Value::Array(a), Value::Array(b)] => a == b,
            [Value::Lambda(a), Value::Lambda(b)] => a == b,
            [Value::Type(a), Value::Type(b)] => a == b,
            [Value::Null, _] => true,
            [_, Value::Null] => true,
            _ => false,
        }
    }
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
