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

    #[test]
    fn test_array() {
        let src = r#"y = 1 * 1
array_1 = [y, 2, 3]
array_2 = [3, 4, 5, 4 - array_1[2]]
array_3 = [1+1, 4 * 2, 0, 0, 0, 0]
exit array_1[0] + array_2[3] + array_3[1]    "#;
        assert_eq!(interpret_zeblang(src), "10".to_string())
    }

    #[test]
    fn test_for() {
        let src = r#"sum = 0
for num in [1, 2, 3]
  sum = sum + num
rof 
prod = 1
for num in [1, 1, 1, 2]
  prod = prod * num
rof
exit prod + sum"#;
        assert_eq!(interpret_zeblang(src), "0".to_string())
    }

    #[test]
    fn test_func() {
        let src = r#"c = 32
a = 40
foo blah(alpha, beta)
  return alpha + beta
oof 
foo sum(a, b, c, d, e)
  i = 4 
  sum = 0 
  nums = [a, b, c, d, e]
  while i + 1
    sum = sum + nums[i]
    i = i - 1
  elihw
  return sum
oof
foo recursive(a) 
    if a == 3
        return a 
    fi 
    a = a + 1
    return recursive(a)
oof
foo main()
  _ = 1 + 1
  return blah(1, 2) + sum(1, 2, 3 * 1, 4, 5) + recursive(0)
oof
exit main()"#;
        assert_eq!(interpret_zeblang(src), "21".to_string())
    }

    #[test]
    fn test_if() {
        let src = r#"ex = 1
why = 2
x = 0
y = 21
if ex == why
  x = y
fi
if y != x 
  x = 3
  y = y - 19
fi 
exit x + y
"#;
        assert_eq!(interpret_zeblang(src), "5".to_string())
    }

    #[test]
    fn test_mut_array() {
        let src = r#"array_1 = [1, 2, 3, 4]
array_2 = [0, 2, 3, 4, 1]
array_1[2 + array_2[4]] = array_1[1] * 4
exit array_1[3] 
"#;
        assert_eq!(interpret_zeblang(src), "8".to_string())
    }

    #[test]
    fn test_precedence() {
        let src = r#"x = 1 + 2 * 3 + 1 * 2 * 1
y = 4 * 1 + 2 * 1 - 2 * 1
z = 1 + 1 - 1 + 1 - 1 * 1
exit x + y * z
"#;
        assert_eq!(interpret_zeblang(src), "13".to_string())
    }
}
