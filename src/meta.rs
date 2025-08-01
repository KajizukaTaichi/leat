use crate::*;

impl Expr {
    /// replace AST(abstract syntax tree) node in meta programming
    pub fn replace(&self, from: &Expr, to: &Expr) -> Expr {
        if from == self {
            to.clone()
        } else {
            match self {
                Expr::Let(name, value, expr) => Expr::Let(
                    Box::new(name.replace(from, to)),
                    Box::new(value.replace(from, to)),
                    Box::new(expr.replace(from, to)),
                ),
                Expr::Call(func, args) => Expr::Call(
                    Box::new(func.replace(from, to)),
                    Box::new(args.replace(from, to)),
                ),
                Expr::Try(risky, callback) => Expr::Try(
                    Box::new(risky.replace(from, to)),
                    Box::new(callback.replace(from, to)),
                ),
                Expr::If(cond, then, els) => Expr::If(
                    Box::new(cond.replace(from, to)),
                    Box::new(then.replace(from, to)),
                    Box::new(els.replace(from, to)),
                ),
                Expr::Array(array) => {
                    Expr::Array(array.iter().map(|x| x.replace(from, to)).collect())
                }
                Expr::Variable(name) => Expr::Variable(name.to_owned()),
                Expr::Literal(Value::Lambda(Lambda::UserDefined(arg, body, env))) => {
                    Expr::Literal(Value::Lambda(Lambda::UserDefined(
                        arg.to_string(),
                        Box::new(body.replace(from, to)),
                        env.clone(),
                    )))
                }
                Expr::Literal(value) => Expr::Literal(value.to_owned()),
            }
        }
    }
}
