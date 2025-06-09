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
                    Value::Lambda(_) => Ok(Value::Type(Type::Lambda)),
                    Value::Type(_) => Ok(Value::Type(Type::Kind)),
                },
                IndexMap::new(),
            )),
        ),
        (
            String::from("ast-replace"),
            Value::Lambda(Lambda::BuiltIn(
                |a, mut env| {
                    Ok(Value::Lambda(Lambda::BuiltIn(
                        |b, mut env| {
                            Ok(Value::Lambda(Lambda::BuiltIn(
                                |c, mut env| {
                                    let Some(Value::Lambda(Lambda::UserDefined(a_a, a, a_c))) =
                                        env.get("a")
                                    else {
                                        return Err(LeatError::InvalidOperation);
                                    };
                                    let Some(Value::Lambda(Lambda::UserDefined(b_a, b, b_c))) =
                                        env.get("a")
                                    else {
                                        return Err(LeatError::InvalidOperation);
                                    };
                                    let Some(Value::Lambda(Lambda::UserDefined(c_a, c, c_c))) =
                                        env.get("b")
                                    else {
                                        return Err(LeatError::InvalidOperation);
                                    };
                                    todo!()
                                },
                                {
                                    env.insert(String::from("b"), b);
                                    env
                                },
                            )))
                        },
                        {
                            env.insert(String::from("a"), a);
                            env
                        },
                    )))
                },
                IndexMap::new(),
            )),
        ),
    ])
}
