#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(f64),
    String(String),
    Bool(bool),
    Ident(String),
    Nest(Vec<Token>),
    Let,
    Assign,
    In,
    If,
    Then,
    Else,
}

impl Token {
    fn new(token: String) -> Option<Token> {
        Some(if token == "let" {
            Token::Let
        } else if token == "=" {
            Token::Assign
        } else if token == "in" {
            Token::In
        } else if token == "if" {
            Token::If
        } else if token == "then" {
            Token::Then
        } else if token == "else" {
            Token::Else
        } else if let Ok(b) = token.parse::<bool>() {
            Token::Bool(b)
        } else if let Ok(n) = token.parse::<f64>() {
            Token::Number(n)
        } else if let Some(Some(string)) = token.strip_prefix("\"").map(|x| x.strip_suffix("\"")) {
            Token::String(string.to_string())
        } else if let Some(Some(nest)) = token.strip_prefix("(").map(|x| x.strip_suffix(")")) {
            Token::Nest(lex(nest)?)
        } else {
            Token::Ident(token)
        })
    }
}

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;
    let mut is_escape = false;

    for c in input.chars() {
        if is_escape {
            current_token.push(match c {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                _ => c,
            });
            is_escape = false;
        } else {
            match c {
                '(' | '{' | '[' if !in_quote => {
                    current_token.push(c);
                    in_parentheses += 1;
                }
                ')' | '}' | ']' if !in_quote => {
                    current_token.push(c);
                    in_parentheses.checked_sub(1).map(|x| in_parentheses = x);
                }
                '"' | '\'' | '`' => {
                    in_quote = !in_quote;
                    current_token.push(c);
                }
                '\\' if in_quote => {
                    current_token.push(c);
                    is_escape = true;
                }
                other => {
                    if other.is_whitespace() && !in_quote {
                        if in_parentheses != 0 {
                            current_token.push(c);
                        } else if !current_token.is_empty() {
                            tokens.push(Token::new(current_token.clone())?);
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c);
                    }
                }
            }
        }
    }

    // Syntax error check
    if is_escape || in_quote || in_parentheses != 0 {
        return None;
    }
    if !current_token.is_empty() {
        tokens.push(Token::new(current_token.clone())?);
        current_token.clear();
    }

    Some(tokens)
}
