#[derive(Debug)]
pub enum TokenKind {
    Exit,
    Assign,
    EndLine,
    VarName(String),
    Int(String),
    Operator(Infix),
}

// dont know if we really nead this. we can just match when parsing
#[derive(Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
}

fn tokenize_str(token_str: &str) -> TokenKind {
    match token_str {
        "exit" => TokenKind::Exit,
        "=" => TokenKind::Assign,
        ";" => TokenKind::EndLine,
        "+" | "-" | "/" | "*" => TokenKind::Operator(tokenize_infix_operators(token_str)),
        value if value.chars().all(char::is_numeric) => TokenKind::Int(value.to_string()),
        value if value.chars().all(char::is_alphanumeric) => TokenKind::VarName(value.to_string()),
        bad_token => panic!("bad token {}", bad_token),
    }
}

fn tokenize_infix_operators(operator: &str) -> Infix {
    match operator {
        "+" => Infix::Plus,
        "-" => Infix::Minus,
        "/" => Infix::Divide,
        "*" => Infix::Multiply,
        bad_token => panic!("bad token {}", bad_token),
    }
}

pub fn tokenize(code: String) -> Vec<TokenKind> {
    code.split_ascii_whitespace().map(tokenize_str).collect()
}
