use std::io::Result;

mod tokenizer;
use tokenizer::Lexer;

mod error;

pub mod parser;
pub use self::parser::ExpressionNode;
pub use self::parser::StatementNode;
use parser::parse;

pub fn make_parsetree(src: String) -> Result<Vec<StatementNode>> {
    let lines: Vec<String> = src.lines().into_iter().map(|s| s.to_string()).collect();
    lines
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(Lexer::lex)
        .enumerate()
        .map(|(line_num, line)| parse(line?, line_num + 1))
        .collect()
}
