use std::io::Result;

use wasm_bindgen::prelude::*;
mod tokenizer;
use tokenizer::Lexer;

mod interpret;
use interpret::interpret;

mod error;

mod parser;
use parser::{parse, StatementNode};

mod printing;

fn make_parsetree(src: String) -> Result<Vec<StatementNode>> {
    let lines: Vec<String> = src.lines().into_iter().map(|s| s.to_string()).collect();
    lines
        .into_iter()
        .filter(|line| !line.trim().is_empty())
        .map(Lexer::lex)
        .enumerate()
        .map(|(line_num, line)| parse(line?, line_num + 1))
        .collect()
}

//TODO need a way to catch infinte loops
#[wasm_bindgen]
pub fn interpret_zeblang(src: &str) -> String {
    match make_parsetree(src.to_string()) {
        Ok(out) => match interpret(out) {
            Ok(value) => format!("{}", value),
            Err(error) => error,
        },
        Err(_) => "error".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::interpret_zeblang;

    #[test]
    fn test_while() {
        let src = r#"
i = 10
out = 0
while i
  out = out + 2
  j = 2
  while j 
    out = out + 1
    j = j -1 
  elihw
  i = i - 1
elihw
exit out
"#;
        assert_eq!(interpret_zeblang(src), "40".to_string());
    }
}
