use crate::*;

impl Expr {
    fn eval(&self, env: &mut Env) -> Option<Value> {
        match self {
            Expr::Literal(value) => Some(value.to_owned()),
            Expr::Variable(name) => env.get(name).cloned(),
            Expr::Call(func, arg) => {
                let Value::Lambda(lambda) = func.eval(env)? else {
                    return None;
                };
                match lambda {
                    Lambda::BuiltIn(body, func_env) => body(arg.eval(env)?, func_env),
                    Lambda::UserDefined(arg_name, body, mut func_env) => {
                        func_env.insert(arg_name, arg.eval(env)?);
                        body.eval(&mut func_env)
                    }
                }
            }
            Expr::Let(name, value, after_expr) => match *name.clone() {
                Expr::Variable(name) => {
                    env.insert(name.to_owned(), value.eval(env)?);
                    after_expr.eval(env)
                }
                Expr::Call(name, arg) => {
                    let Expr::Variable(arg) = *arg else {
                        return None;
                    };
                    let func = Lambda::UserDefined(arg, value.to_owned(), env.clone());
                    let func = Box::new(Expr::Literal(Value::Lambda(func)));
                    Expr::Let(name, func, after_expr.clone()).eval(env)
                }
                _ => None,
            },
        }
    }
}
