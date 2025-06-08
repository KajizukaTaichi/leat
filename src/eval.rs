use crate::*;

impl Expr {
    pub fn eval(&self, env: &mut Env) -> Result<Value, LeatError> {
        match self {
            Expr::Variable(name) => {
                if let Some(value) = env.get(name) {
                    Ok(value.to_owned())
                } else {
                    Err(LeatError::UndefinedName(name.to_string()))
                }
            }
            Expr::Literal(value) => match value {
                Value::Lambda(Lambda::UserDefined(arg, body, _)) => Ok(Value::Lambda(
                    Lambda::UserDefined(arg.to_string(), body.clone(), env.clone()),
                )),
                _ => Ok(value.to_owned()),
            },
            Expr::Call(func, arg) => {
                let Value::Lambda(lambda) = func.eval(env)? else {
                    return Err(LeatError::CallNotLambda(*func.clone()));
                };
                match lambda {
                    Lambda::BuiltIn(body, func_env) => body(arg.eval(env)?, func_env),
                    Lambda::UserDefined(arg_name, body, mut func_env) => {
                        func_env.extend(env.clone());
                        func_env.insert(arg_name, arg.eval(env)?);
                        body.eval(&mut func_env)
                    }
                }
            }
            Expr::Let(name, value, after_expr) => match *name.clone() {
                Expr::Variable(name) => {
                    let value = value.eval(env)?;
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
                if let Value::Bool(true) = cond.eval(env)? {
                    then.eval(env)
                } else {
                    els.eval(env)
                }
            }
        }
    }
}

pub enum LeatError {
    UndefinedName(String),
    InvalidBind(Expr),
    InvalidArg(Expr),
    CallNotLambda(Expr),
}
