use std::io::Result;

mod tokenizer;
use error::new_error;
use tokenizer::Lexer;

mod local_client;
use local_client::{read_file, write_assembly_file, write_json, write_llvm_file};

mod parser;
use parser::{parse, StatementNode};

mod error;

mod generator;
use generator::Generator;

mod llvm_generator;
use llvm_generator::LlvmGenerator;

mod arg_parser;
use arg_parser::{parse_args, Arg, TargetKind};

fn main() -> Result<()> {
    let args = parse_args();
    if let Arg::Filename(filename) = args.get("filename").ok_or(new_error("incorrect usage"))? {
        let code = read_file(filename);

        let parse_tree: Result<Vec<StatementNode>> = code
            .into_iter()
            .filter(|line| !line.trim().is_empty())
            .map(Lexer::lex)
            .enumerate()
            .map(|(line_num, line)| parse(line?, line_num + 1))
            .collect();

        match args.get("target") {
            Some(val) => {
                if let Arg::Target(t) = val {
                    dbg!(t);
                    match t {
                        TargetKind::Json => write_json(filename, parse_tree)?,
                        TargetKind::Llvm => {
                            println!("YESSS");
                            write_llvm_file(&filename, LlvmGenerator::new().generate(parse_tree?)?)?
                        }
                    }
                }
            }
            None => {
                let mut generator = Generator::new();
                let assembly = generator.generate(parse_tree?);
                write_assembly_file(&filename, assembly?)?;
            }
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests;
