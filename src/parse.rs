use crate::{lexer::text_escape, *};

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
        } else if let Token::Lambda = tokens.first()? {
            let tokens = tokens.get(1..)?;
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Dot).collect();
            let body = tokens.get(1..)?.join(&Token::Dot);
            let body = Box::new(Expr::parse(body)?);
            let arg = tokens.first()?.to_vec();
            let [Token::Ident(arg)] = arg.as_slice() else {
                return None;
            };
            let lambda = Lambda::UserDefined(arg.to_string(), body, IndexMap::new());
            Some(Expr::Literal(Value::Lambda(lambda)))
        } else if let Token::Try = tokens.first()? {
            let tokens = tokens.get(1..)?;
            let tokens: Vec<&[Token]> = tokens.split(|x| *x == Token::Catch).collect();
            let trys = tokens.first()?.to_vec();
            let trys = Box::new(Expr::parse(trys)?);
            let catch = tokens.get(1..)?.join(&Token::Catch);
            let catch = Box::new(Expr::parse(catch)?);
            Some(Expr::Try(trys, catch))
        } else if let [Token::Number(a), Token::Dot, Token::Number(b)] = tokens.as_slice() {
            let number = format!("{a}.{b}").parse::<f64>().unwrap();
            Some(Expr::Literal(Value::Number(number)))
        } else if tokens.len() >= 2 {
            for i in 2..=tokens.len() - 1 {
                if let Token::Ident(operator) = tokens.get(tokens.len() - i)? {
                    if operator.chars().all(|c| c.is_ascii_punctuation()) {
                        if let [Some(lhs), Some(rhs)] = [
                            Expr::parse(tokens.get(..tokens.len() - i)?.to_vec()),
                            Expr::parse(tokens.get(tokens.len() - i + 1..)?.to_vec()),
                        ] {
                            return Some(Expr::Call(
                                Box::new(Expr::Call(
                                    Box::new(Expr::Variable(operator.to_owned())),
                                    Box::new(lhs),
                                )),
                                Box::new(rhs),
                            ));
                        }
                    }
                }
            }
            let func = Expr::parse(tokens.get(..tokens.len() - 1)?.to_vec())?;
            let args = Expr::parse(vec![tokens.last()?.clone()])?;
            Some(Expr::Call(Box::new(func), Box::new(args)))
        } else {
            match tokens.first()? {
                Token::Number(n) => Some(Expr::Literal(Value::Number(*n))),
                Token::String(s) => Some(Expr::Literal(Value::String(text_escape(s)))),
                Token::Bool(b) => Some(Expr::Literal(Value::Bool(*b))),
                Token::Type(t) => Some(Expr::Literal(Value::Type(t.clone()))),
                Token::Ident(name) => Some(Expr::Variable(name.to_owned())),
                Token::Nest(tokens) => Some(Expr::parse(tokens.to_vec())?),
                Token::Array(tokens) => Some(Expr::Array(
                    tokens
                        .split(|x| *x == Token::Comma)
                        .map(|x| Expr::parse(x.to_vec()))
                        .collect::<Option<Vec<_>>>()?,
                )),
                _ => None,
            }
        }
    }
}
