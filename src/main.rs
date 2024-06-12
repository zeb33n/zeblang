use std::io::Result;

mod tokenizer;
use error::new_error;
use tokenizer::{Lexer, TokenKind};

mod local_client;
use local_client::{read_file, write_assembly_file, write_json};

mod parser;
use parser::{Parser, StatementNode};

mod error;

mod generator;
use generator::Generator;

mod arg_parser;
use arg_parser::parse_args;

// loop through args so order soesnt matter
fn main() -> Result<()> {
    let args = parse_args();
    let filename = args.get("filename").ok_or(new_error("incorrect usage"))?;

    let code = read_file(filename);
    // collect the errors into a vec of errors
    let token_vecs = code
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(Lexer::lex)
        .collect::<Result<Vec<Vec<TokenKind>>>>()?;

    let mut parse_tree: Vec<StatementNode> = Vec::with_capacity(token_vecs.len());
    let mut parse_errors: Vec<String> = Vec::with_capacity(token_vecs.len());
    for (i, line) in token_vecs.into_iter().enumerate() {
        match Parser::parse(line, i + 1) {
            Ok(statement) => parse_tree.push(statement),
            Err(e) => parse_errors.push(e.to_string()),
        }
    }

    match (
        parse_errors.as_slice(),
        parse_tree.as_slice(),
        args.get("json"),
    ) {
        (_, _, Some(_)) => write_json(filename, parse_tree)?,
        ([], _, None) => {
            let assembly = Generator::generate(parse_tree);
            write_assembly_file(&filename, assembly?)?;
        }
        (_, _, None) => {
            parse_errors.into_iter().for_each(|e| println!("{}", e));
            Err(new_error("Syntax errors!"))?;
            todo!("code to print the errors");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
