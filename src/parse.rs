use crate::*;

impl Expr {
    fn parse(tokens: Vec<Token>) -> Option<Expr> {
        if let Token::Let = tokens.first()? {
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Assign).collect();
            let [name, tokens] = [tokens.first()?.to_vec(), tokens.get(1..)?.concat()];
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::In).collect();
            let [value, after_expr] = [tokens.first()?.to_vec(), tokens.get(1..)?.concat()];
            Some(Expr::Let(
                Box::new(Expr::parse(name)?),
                Box::new(Expr::parse(value)?),
                Box::new(Expr::parse(after_expr)?),
            ))
        } else {
            None
        }
    }
}
