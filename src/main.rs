use std::env;
use std::io::Result;

mod tokenizer;
use tokenizer::{tokenize, TokenKind};

mod local_client;
use local_client::{read_file, write_assembly_file};

mod parser;
use parser::parse;

mod error;

fn tokens_to_assembly(line: Vec<TokenKind>) -> String {
    let mut output = String::from("global _start\n_start:\n");
    match &line[..] {
        [.., TokenKind::Return, TokenKind::Int(value), TokenKind::EndLine] => {
            output += "   mov rax, 60\n";
            output += format!("   mov rdi, {}\n", value).as_str();
            output += "   syscall";
        }
        _ => panic!("syntax error"),
    }
    output
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = match &args[..] {
        [_, filename] => filename,
        _ => panic!("incorrect usage. correct usage is: \nzeb <file.zb>"),
    };
    let code = read_file(filename);
    dbg!(parse(tokenize(code)));
    //write_assembly_file(&filename, tokens_to_assembly(tokenize(code)))?;
    Ok(())
}
