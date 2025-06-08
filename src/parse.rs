use crate::*;

impl Expr {
    pub fn parse(tokens: Vec<Token>) -> Option<Expr> {
        if let Token::Let = tokens.first()? {
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Assign).collect();
            let [name, tokens] = [tokens.first()?.to_vec(), tokens.get(1..)?.concat()];
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::In).collect();
            let [value, after_expr] = [tokens.first()?.to_vec(), tokens.get(1..)?.concat()];
            let name = Box::new(Expr::parse(name)?);
            let value = Box::new(Expr::parse(value)?);
            let after_expr = Box::new(Expr::parse(after_expr)?);
            Some(Expr::Let(name, value, after_expr))
        } else if let Token::If = tokens.first()? {
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Then).collect();
            let [cond, tokens] = [tokens.first()?.to_vec(), tokens.get(1..)?.concat()];
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Else).collect();
            let [then, els] = [tokens.first()?.to_vec(), tokens.get(1..)?.concat()];
            let cond = Box::new(Expr::parse(cond)?);
            let then = Box::new(Expr::parse(then)?);
            let els = Box::new(Expr::parse(els)?);
            Some(Expr::If(cond, then, els))
        } else {
            None
        }
    }
}
