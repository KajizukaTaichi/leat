use crate::*;

pub fn stdlib() -> Env {
    macro_rules! curry_2arg {
        ($processing: expr) => {
            Value::Lambda(Lambda::BuiltIn(
                |a, mut env| {
                    Ok(Value::Lambda(Lambda::BuiltIn(
                        |b, env| {
                            let a = env.get("a").unwrap().clone();
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
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a + b)),
                [Value::String(a), Value::String(b)] => Ok(Value::String(a + &b)),
                [Value::Array(a), Value::Array(b)] => Ok(Value::Array([a, b].concat())),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("-"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a - b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("*"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a * b)),
                [Value::String(a), Value::Number(b)] => Ok(Value::String(a.repeat(b as usize))),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("/"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a / b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("=="),
            curry_2arg!(|a, b| Ok(Value::Bool(a == b))),
        ),
        (
            String::from(">"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a > b)),
                [Value::String(a), Value::String(b)] => Ok(Value::Bool(a > b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("<"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a < b)),
                [Value::String(a), Value::String(b)] => Ok(Value::Bool(a < b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("&"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Bool(a), Value::Bool(b)] => Ok(Value::Bool(a & b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("|"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Bool(a), Value::Bool(b)] => Ok(Value::Bool(a | b)),
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("cast"),
            curry_2arg!(|a, b| match [a, b] {
                [Value::Number(a), Value::Type(Type::String)] => {
                    Ok(Value::String(a.to_string()))
                }
                [Value::String(a), Value::Type(Type::Number)] => {
                    if let Ok(n) = a.parse::<f64>() {
                        Ok(Value::Number(n))
                    } else {
                        Err(LeatError::InvalidOperation)
                    }
                }
                _ => Err(LeatError::InvalidOperation),
            }),
        ),
        (
            String::from("typeof"),
            Value::Lambda(Lambda::BuiltIn(
                |a, _| match a {
                    Value::Number(_) => Ok(Value::Type(Type::Number)),
                    Value::String(_) => Ok(Value::Type(Type::String)),
                    Value::Bool(_) => Ok(Value::Type(Type::Bool)),
                    Value::Array(_) => Ok(Value::Type(Type::Array)),
                    Value::Lambda(_) => Ok(Value::Type(Type::Lambda)),
                    Value::Type(_) => Ok(Value::Type(Type::Kind)),
                },
                IndexMap::new(),
            )),
        ),
        (
            String::from("ast-replace"),
            Value::Lambda(Lambda::BuiltIn(
                |expr, mut env| {
                    Ok(Value::Lambda(Lambda::BuiltIn(
                        |from, mut env| {
                            Ok(Value::Lambda(Lambda::BuiltIn(
                                |c, env| {
                                    let Some(Value::Lambda(Lambda::UserDefined(
                                        arg,
                                        expr,
                                        expr_env,
                                    ))) = env.get("expr")
                                    else {
                                        return Err(LeatError::InvalidOperation);
                                    };
                                    let Some(Value::Lambda(Lambda::UserDefined(_, from, _))) =
                                        env.get("from")
                                    else {
                                        return Err(LeatError::InvalidOperation);
                                    };
                                    let Value::Lambda(Lambda::UserDefined(_, to, _)) = c else {
                                        return Err(LeatError::InvalidOperation);
                                    };
                                    Ok(Value::Lambda(Lambda::UserDefined(
                                        arg.to_string(),
                                        Box::new(expr.replace(from, &*to.clone())),
                                        expr_env.clone(),
                                    )))
                                },
                                {
                                    env.insert(String::from("from"), from);
                                    env
                                },
                            )))
                        },
                        {
                            env.insert(String::from("expr"), expr);
                            env
                        },
                    )))
                },
                IndexMap::new(),
            )),
        ),
    ])
}
