use std::{io::Result, process::exit};

mod tokenizer;
use error::new_error;
use tokenizer::Lexer;

mod local_client;
use local_client::{read_file, write_assembly_file, write_json};

mod parser;
use parser::{parse, StatementNode};

mod error;

mod generator;
use generator::Generator;

mod arg_parser;
use arg_parser::parse_args;

mod interpret;
use interpret::interpret;

mod printing;

// loop through args so order soesnt matter
fn main() -> Result<()> {
    let args = parse_args();
    let filename = args.get("filename").ok_or(new_error("incorrect usage"))?;

    let code = read_file(filename);
    // collect the errors into a vec of errors
    let parse_tree: Result<Vec<StatementNode>> = code
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(Lexer::lex)
        .enumerate()
        .map(|(line_num, line)| parse(line?, line_num + 1))
        .collect();

    if let Some(_) = args.get("json") {
        write_json(filename, parse_tree)?;
    } else if let Some(_) = args.get("interpret") {
        match interpret(parse_tree?) {
            Ok(value) => exit(value),
            Err(error) => return Err(new_error(&error)),
        }
    } else {
        let mut generator = Generator::new();
        let assembly = generator.generate(parse_tree?);
        write_assembly_file(&filename, assembly?)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
