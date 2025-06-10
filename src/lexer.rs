use crate::*;

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
            is_escape = false;
            match c {
                '(' | '[' if !in_quote => {
                    current_token.push(c);
                    in_parentheses += 1;
                }
                ')' | ']' if !in_quote => {
                    current_token.push(c);
                    in_parentheses.checked_sub(1).map(|x| in_parentheses = x);
                }
                '"' => {
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
                '`' => {
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

pub fn f_string(input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;

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
            is_escape = false;
            match c {
                '{' if in_parentheses == 0 => {
                    tokens.push(Token::String(current_token.clone()));
                    current_token.clear();
                    in_parentheses += 1;
                }
                '}' => {
                    in_parentheses.checked_sub(1).map(|x| in_parentheses = x);
                    if in_parentheses == 0 {
                        tokens.push(Token::Nest(lex(&current_token)?));
                    }
                }
                '`' => {
                    current_token.push(c);
                    is_escape = true;
                }
                _ => current_token.push(c),
            }
        }
    }

    // Syntax error check
    if is_escape || in_parentheses != 0 {
        return None;
    }
    if !current_token.is_empty() {
        tokens.push(Token::String(current_token.clone()));
        current_token.clear();
    }

    Some(tokens)
}

pub fn text_escape(text: &str) -> String {
    let mut result = String::new();
    let mut is_escape = false;
    for c in text.chars() {
        if is_escape {
            result.push(c);
            is_escape = false;
        } else {
            match c {
                '`' => {
                    is_escape = true;
                }
                _ => result.push(c),
            }
        }
    }
    result
}
