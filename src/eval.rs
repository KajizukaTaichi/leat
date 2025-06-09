use crate::*;

impl Expr {
    pub fn eval(&self, env: &mut Env) -> Result<Value, LeatError> {
        match self {
            Expr::Variable(name) => {
                if let Some(value) = env.get(name) {
                    Ok(value.to_owned())
                } else {
                    Err(LeatError::Undefined(name.to_string()))
                }
            }
            Expr::Literal(value) => match value {
                Value::Lambda(Lambda::UserDefined(arg, body, _)) => Ok(Value::Lambda(
                    Lambda::UserDefined(arg.to_string(), body.clone(), env.clone()),
                )),
                _ => Ok(value.to_owned()),
            },
            Expr::Array(array) => Ok(Value::Array(
                array
                    .iter()
                    .map(|x| x.eval(env))
                    .collect::<Result<Vec<_>, LeatError>>()?,
            )),
            Expr::Call(func, arg) => {
                let Value::Lambda(lambda) = func.eval(env)? else {
                    return Err(LeatError::NonLambda(*func.clone()));
                };
                match lambda {
                    Lambda::BuiltIn(body, func_env) => {
                        let mut env = env.clone();
                        env.extend(func_env.clone());
                        body(arg.eval(&mut env)?, env.clone())
                    }
                    Lambda::UserDefined(arg_name, body, func_env) => {
                        let mut env = env.clone();
                        env.extend(func_env.clone());
                        let value = arg.eval(&mut env)?;
                        env.insert(arg_name, value);
                        body.eval(&mut env)
                    }
                }
            }
            Expr::Let(name, value, after_expr) => match *name.clone() {
                Expr::Variable(name) => {
                    let value = value.eval(&mut env.clone())?;
                    if env.contains_key(&name) {}
                    env.insert(name.to_owned(), value);
                    after_expr.eval(env)
                }
                Expr::Call(name, arg) => {
                    let Expr::Variable(arg) = *arg else {
                        return Err(LeatError::InvalidArg(*arg));
                    };
                    let func = Lambda::UserDefined(arg, value.to_owned(), env.clone());
                    let func = Box::new(Expr::Literal(Value::Lambda(func)));
                    Expr::Let(name, func, after_expr.clone()).eval(env)
                }
                _ => Err(LeatError::InvalidBind(*name.clone())),
            },
            Expr::If(cond, then, els) => {
                if let Ok(Value::Bool(false)) | Err(_) = cond.eval(env) {
                    els.eval(env)
                } else {
                    then.eval(env)
                }
            }
            Expr::Try(risky, callback) => match risky.eval(env) {
                Ok(result) => Ok(result),
                Err(_) => callback.eval(env),
            },
        }
    }
}
