use std::env;
use std::io::Result;

mod tokenizer;
use tokenizer::Lexer;

mod local_client;
use local_client::{read_file, write_assembly_file};

mod parser;
use parser::{parse, StatementNode};

mod error;

mod generator;
use generator::Generator;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = match &args[..] {
        [_, filename] => filename,
        _ => panic!("incorrect usage. correct usage is: \nzeb <file.zb>"),
    };
    let code = read_file(filename);
    let parse_tree: Result<Vec<StatementNode>> = code
        .into_iter()
        .map(|line| Ok(parse(Lexer::lex(line)?)?))
        .collect();
    let mut generator = Generator::new();
    let assembly = generator.generate(parse_tree?);
    write_assembly_file(&filename, assembly)?;
    Ok(())
}

#[cfg(test)]
mod tests;
