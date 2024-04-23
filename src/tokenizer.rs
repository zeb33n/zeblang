#[derive(Debug)]
pub enum TokenKind {
    Exit,
    For,
    EndFor,
    While,
    EndWhile,
    In,
    Assign,
    EndLine,
    OpenParen,
    CloseParen,
    VarName(String),
    Int(String),
    Operator(String),
    Callable(String),
}

fn tokenize_str(token_str: &str) -> TokenKind {
    match token_str {
        "=" => TokenKind::Assign,
        ";" => TokenKind::EndLine,
        "(" => TokenKind::OpenParen,
        ")" => TokenKind::CloseParen,
        "for" => TokenKind::For,
        "rof" => TokenKind::EndFor,
        "while" => TokenKind::While,
        "elihw" => TokenKind::EndWhile,
        "in" => TokenKind::In,
        "exit" => TokenKind::Exit,
        "+" | "-" | "/" | "*" => TokenKind::Operator(token_str.to_string()),
        value if value.chars().all(char::is_numeric) => TokenKind::Int(value.to_string()),
        value if value.chars().all(char::is_alphanumeric) => TokenKind::VarName(value.to_string()),
        // bug if there is ( in middle of word and at the end
        value if value.chars().all(|c| c.is_alphanumeric() | (c == '(')) & value.ends_with('(') => {
            TokenKind::Callable(value.to_string())
        }
        bad_token => panic!("bad token {}", bad_token),
    }
}

pub fn tokenize(code: String) -> Vec<TokenKind> {
    code.split_ascii_whitespace().map(tokenize_str).collect()
}
