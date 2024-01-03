#[derive(Debug, Clone)]
pub enum TokenKind {
    Return,
    Assign,
    EndLine,
    VarName(String),
    Int(String),
    Operator(String),
}

fn tokenize_str(token_str: &str) -> TokenKind {
    match token_str {
        "return" => TokenKind::Return,
        "=" => TokenKind::Assign,
        ";" => TokenKind::EndLine,
        "+" | "-" | "/" | "*" => TokenKind::Operator(token_str.to_string()),
        value if value.chars().all(char::is_numeric) => TokenKind::Int(value.to_string()),
        value if value.chars().all(char::is_alphanumeric) => TokenKind::VarName(value.to_string()),
        bad_token => panic!("bad token {}", bad_token),
    }
}

pub fn tokenize(code: String) -> Vec<TokenKind> {
    code.split_ascii_whitespace().map(tokenize_str).collect()
    //code.into_iter()
    //.map(|line| line.split_ascii_whitespace().map(tokenize_str).collect())
    //.collect()
}
