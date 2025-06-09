use crate::*;

pub fn lex(input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
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
            '\\' if in_parentheses == 0 && current_token.is_empty() => {
                tokens.push(Token::Lambda);
            }
            '.' if in_parentheses == 0 => {
                if !current_token.is_empty() {
                    tokens.push(Token::new(current_token.clone())?);
                    current_token.clear();
                }
                tokens.push(Token::Dot);
            }
            ',' if in_parentheses == 0 => {
                if !current_token.is_empty() {
                    tokens.push(Token::new(current_token.clone())?);
                    current_token.clear();
                }
                tokens.push(Token::Comma);
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

    // Syntax error check
    if in_quote || in_parentheses != 0 {
        return None;
    }
    if !current_token.is_empty() {
        tokens.push(Token::new(current_token.clone())?);
        current_token.clear();
    }

    Some(tokens)
}
