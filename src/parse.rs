use crate::*;

impl Expr {
    pub fn parse(tokens: Vec<Token>) -> Option<Expr> {
        if let Token::Let = tokens.first()? {
            let tokens = tokens.get(1..)?;
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Assign).collect();
            let name = tokens.first()?.to_vec();
            let tokens = tokens.get(1..)?.join(&Token::Assign);
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::In).collect();
            let value = tokens.first()?.to_vec();
            let after_expr = tokens.get(1..)?.join(&Token::In);
            let name = Box::new(Expr::parse(name)?);
            let value = Box::new(Expr::parse(value)?);
            let after_expr = Box::new(Expr::parse(after_expr)?);
            Some(Expr::Let(name, value, after_expr))
        } else if let Token::If = tokens.first()? {
            let tokens = tokens.get(1..)?;
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Then).collect();
            let cond = tokens.first()?.to_vec();
            let tokens = tokens.get(1..)?.join(&Token::Then);
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Else).collect();
            let then = tokens.first()?.to_vec();
            let els = tokens.get(1..)?.join(&Token::Else);
            let cond = Box::new(Expr::parse(cond)?);
            let then = Box::new(Expr::parse(then)?);
            let els = Box::new(Expr::parse(els)?);
            Some(Expr::If(cond, then, els))
        } else if tokens.len() >= 2 {
            let func = Expr::parse(tokens.get(..tokens.len() - 1)?.to_vec())?;
            let args = Expr::parse(vec![tokens.last()?.clone()])?;
            Some(Expr::Call(Box::new(func), Box::new(args)))
        } else {
            match tokens.first()? {
                Token::Number(b) => Some(Expr::Literal(Value::Number(*b))),
                Token::String(b) => Some(Expr::Literal(Value::String(b.to_owned()))),
                Token::Bool(b) => Some(Expr::Literal(Value::Bool(*b))),
                Token::Ident(name) => Some(Expr::Variable(name.to_owned())),
                Token::Nest(tokens) => Some(Expr::parse(tokens.to_vec())?),
                _ => None,
            }
        }
    }
}
