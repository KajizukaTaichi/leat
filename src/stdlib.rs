use crate::*;

pub fn stdlib() -> Env {
    macro_rules! curry_2arg {
        ($processing: expr) => {
            Value::Lambda(Lambda::BuiltIn(
                |a, mut env| {
                    Ok(Value::Lambda(Lambda::BuiltIn(
                        |b, mut env| {
                            let a = env.get("a").unwrap().clone();
                            $processing(a, b, &mut env)
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
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a + b)),
                [Value::String(a), Value::String(b)] => Ok(Value::String(a + &b)),
                [Value::Array(a), Value::Array(b)] => Ok(Value::Array([a, b].concat())),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("-"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a - b)),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("*"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a * b)),
                [Value::String(a), Value::Number(b)] => Ok(Value::String(a.repeat(b as usize))),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("/"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a / b)),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("%"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a % b)),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("=="),
            curry_2arg!(|a, b, _| Ok(Value::Bool(a == b))),
        ),
        (
            String::from(">"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a > b)),
                [Value::String(a), Value::String(b)] => Ok(Value::Bool(a > b)),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("<"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Number(a), Value::Number(b)] => Ok(Value::Bool(a < b)),
                [Value::String(a), Value::String(b)] => Ok(Value::Bool(a < b)),
                _ => Err(LeatError::TypeMismatch(Type::Number)),
            }),
        ),
        (
            String::from("&"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Bool(a), Value::Bool(b)] => Ok(Value::Bool(a & b)),
                _ => Err(LeatError::TypeMismatch(Type::Bool)),
            }),
        ),
        (
            String::from("|"),
            curry_2arg!(|a, b, _| match [a, b] {
                [Value::Bool(a), Value::Bool(b)] => Ok(Value::Bool(a | b)),
                _ => Err(LeatError::TypeMismatch(Type::Bool)),
            }),
        ),
        (
            String::from("cast"),
            curry_2arg!(|a, b, _| match [a, b] {
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
                [Value::String(a), Value::Type(Type::String)] => Ok(Value::String(a)),
                [Value::Number(a), Value::Type(Type::Number)] => Ok(Value::Number(a)),
                _ => Err(LeatError::TypeMismatch(Type::Kind)),
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
                    Value::Null => Ok(Value::Type(Type::Kind)),
                },
                IndexMap::new(),
            )),
        ),
        (
            String::from("car"),
            Value::Lambda(Lambda::BuiltIn(
                |array, _| {
                    let Value::Array(array) = array else {
                        return Err(LeatError::TypeMismatch(Type::Array));
                    };
                    Ok(ok!(array.first())?.clone())
                },
                IndexMap::new(),
            )),
        ),
        (
            String::from("cdr"),
            Value::Lambda(Lambda::BuiltIn(
                |array, _| {
                    let Value::Array(array) = array else {
                        return Err(LeatError::TypeMismatch(Type::Array));
                    };
                    Ok(Value::Array(ok!(array.get(1..))?.to_vec()))
                },
                IndexMap::new(),
            )),
        ),
        (
            String::from("~"),
            curry_2arg!(|a, b, _| {
                let Value::Number(a) = a else {
                    return Err(LeatError::TypeMismatch(Type::Number));
                };
                let Value::Number(b) = b else {
                    return Err(LeatError::TypeMismatch(Type::Number));
                };
                let mut result = vec![];
                for i in a as usize..b as usize {
                    result.push(Value::Number(i as f64));
                }
                Ok(Value::Array(result))
            }),
        ),
        (
            String::from("map"),
            curry_2arg!(|func: Value, array, env: &mut Env| {
                let Value::Array(array) = array else {
                    return Err(LeatError::TypeMismatch(Type::Array));
                };
                Ok(Value::Array(
                    array
                        .iter()
                        .map(|value| {
                            Expr::Call(
                                Box::new(Expr::Literal(func.clone())),
                                Box::new(Expr::Literal(value.clone())),
                            )
                            .eval(env)
                        })
                        .collect::<Result<Vec<Value>, LeatError>>()?,
                ))
            }),
        ),
        (
            String::from("filter"),
            curry_2arg!(|func: Value, array, env: &mut Env| {
                let Value::Array(array) = array else {
                    return Err(LeatError::TypeMismatch(Type::Array));
                };
                Ok(Value::Array(
                    array
                        .iter()
                        .cloned()
                        .filter(|value| {
                            Expr::Call(
                                Box::new(Expr::Literal(func.clone())),
                                Box::new(Expr::Literal(value.clone())),
                            )
                            .eval(env)
                                == Ok(Value::Bool(true))
                        })
                        .collect::<Vec<Value>>(),
                ))
            }),
        ),
        (
            String::from("reduce"),
            curry_2arg!(|func: Value, array, env: &mut Env| {
                let Value::Array(array) = array else {
                    return Err(LeatError::TypeMismatch(Type::Array));
                };
                let mut result = ok!(array.first())?.clone();
                for value in ok!(array.get(1..))? {
                    result = Expr::Call(
                        Box::new(Expr::Call(
                            Box::new(Expr::Literal(func.clone())),
                            Box::new(Expr::Literal(result.clone())),
                        )),
                        Box::new(Expr::Literal(value.clone())),
                    )
                    .eval(env)?;
                }
                Ok(result)
            }),
        ),
        (
            String::from("throw"),
            Value::Lambda(Lambda::BuiltIn(
                |msg, _| {
                    let Value::String(msg) = msg else {
                        return Err(LeatError::TypeMismatch(Type::String));
                    };
                    Err(LeatError::UserDefined(msg))
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
                                        return Err(LeatError::TypeMismatch(Type::Lambda));
                                    };
                                    let Some(Value::Lambda(Lambda::UserDefined(_, from, _))) =
                                        env.get("from")
                                    else {
                                        return Err(LeatError::TypeMismatch(Type::Lambda));
                                    };
                                    let Value::Lambda(Lambda::UserDefined(_, to, _)) = c else {
                                        return Err(LeatError::TypeMismatch(Type::Lambda));
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
        (
            String::from("join"),
            curry_2arg!(|array, del, _| {
                let Value::Array(array) = array else {
                    return Err(LeatError::TypeMismatch(Type::Array));
                };
                let Value::String(del) = del else {
                    return Err(LeatError::TypeMismatch(Type::String));
                };
                let mut string_array = vec![];
                for i in array {
                    let Value::String(i) = i else {
                        return Err(LeatError::TypeMismatch(Type::String));
                    };
                    string_array.push(i);
                }
                Ok(Value::String(string_array.join(&del)))
            }),
        ),
    ])
}
