use std::env;
use std::fs::read_to_string;

#[derive(Debug)]
enum TokenKind {
    Return,
    Assign,
    VarName(String),
    Int(String),
    Operator(String),
}

fn tokenize_str(token_str: &str) -> TokenKind {
    match token_str {
        "return" => TokenKind::Return,
        "=" => TokenKind::Assign,
        "+" | "-" | "/" | "*" => TokenKind::Operator(token_str.to_string()),
        value if value.chars().all(char::is_numeric) => TokenKind::Int(value.to_string()),
        value if value.chars().all(char::is_alphanumeric) => TokenKind::VarName(value.to_string()),
        bad_token => panic!("bad token {}", bad_token), //value => match value.chars().,
    }
}

fn tokenize(code: String) -> Vec<TokenKind> {
    code.split_ascii_whitespace().map(tokenize_str).collect()
}

fn read_lines(filename: &str) -> String {
    match read_to_string(filename) {
        Ok(value) => value,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: String = match &args[..] {
        [_, filename] => read_lines(filename),
        _ => panic!("incorrect usage. correct usage is: \nzeb <file.zb>"),
    };
    dbg!(tokenize(code));
}

//#[derive(Debug)]
//struct Token {
//kind: TokenKind,
//value: String,
//}

//impl Token {
//fn new(kind: TokenKind, value: Option<String>) -> Self {
//Self {
//kind: kind,
//value: value.unwrap_or("".to_string()),
//}
//}
//}
