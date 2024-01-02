use std::env;
use std::fs::{read_to_string, File};
use std::io::{Result, Write};

#[derive(Debug, Clone)]
enum TokenKind {
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
        bad_token => panic!("bad token {}", bad_token), //value => match value.chars().,
    }
}

fn tokenize(code: String) -> Vec<TokenKind> {
    code.split_ascii_whitespace().map(tokenize_str).collect()
}

fn tokens_to_assembly(tokens: Vec<TokenKind>) -> String {
    let mut lines: Vec<Vec<TokenKind>> = Vec::new();
    let mut line = Vec::new();
    for token in tokens.into_iter() {
        match token {
            TokenKind::EndLine => {
                lines.push(line.clone());
                line.retain(|_| false);
            }
            other => line.push(other),
        }
    }
    let mut output = String::from("global _start\n_start:\n");
    for line in lines.into_iter() {
        match &line[..] {
            [TokenKind::Return, TokenKind::Int(value)] => {
                output += "   mov rax, 60\n";
                output += format!("   mov rdi, {}\n", value).as_str();
                output += "   syscall";
            }
            _ => panic!("syntax error"),
        }
    }
    output
}

fn read_file(filename: &str) -> String {
    match read_to_string(filename) {
        Ok(value) => value,
        Err(e) => panic!("Error reading file: {}", e),
    }
}

fn write_assembly_file(filename: &str, body: String) -> Result<()> {
    let mut file = File::create(format!("{}{}", filename.split(".").next().unwrap(), ".asm"))?;
    file.write_all(body.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = match &args[..] {
        [_, filename] => filename,
        _ => panic!("incorrect usage. correct usage is: \nzeb <file.zb>"),
    };
    let code: String = read_file(filename);

    write_assembly_file(&filename, tokens_to_assembly(tokenize(code)))?;
    Ok(())
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
