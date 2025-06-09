use crate::*;

impl Expr {
    /// Beta reduction of constant arguments when apply Function
    pub fn replace(&self, from: &Expr, to: &Expr) -> Expr {
        if from == self {
            to.clone()
        } else {
            match self {
                Expr::Call(func, args) => Expr::Call(func.replace(from, to), args),
            }
        }
    }
}
