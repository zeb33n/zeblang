use std::env;
use std::io::Result;

mod tokenizer;
use tokenizer::{tokenize, TokenKind};

mod local_client;
use local_client::{read_file, write_assembly_file};

mod parser;
use parser::parse;

mod error;

mod generator;
use generator::generate;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = match &args[..] {
        [_, filename] => filename,
        _ => panic!("incorrect usage. correct usage is: \nzeb <file.zb>"),
    };
    let code = read_file(filename);
    write_assembly_file(&filename, generate(parse(tokenize(code))))?;
    Ok(())
}
