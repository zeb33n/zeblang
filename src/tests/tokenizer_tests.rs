use crate::tokenizer::{Lexer, TokenKind};
use std::io::Result;

#[test]
fn test_lexer() -> Result<()> {
    let out = Lexer::lex(" =;()*0123 789 exit rof exit_-z_A01+foo(1+1)".to_string())?;
    let target = vec![
        TokenKind::Assign,
        TokenKind::EndLine,
        TokenKind::OpenParen,
        TokenKind::CloseParen,
        TokenKind::Operator("*".to_string()),
        TokenKind::Int("0123".to_string()),
        TokenKind::Int("789".to_string()),
        TokenKind::Exit,
        TokenKind::EndFor,
        TokenKind::VarName("exit_".to_string()),
        TokenKind::Operator("-".to_string()),
        TokenKind::VarName("z_A01".to_string()),
        TokenKind::Operator("+".to_string()),
        TokenKind::Callable("foo(".to_string()),
        TokenKind::Int("1".to_string()),
        TokenKind::Operator("+".to_string()),
        TokenKind::Int("1".to_string()),
        TokenKind::CloseParen,
    ];
    let _: Vec<_> = out
        .into_iter()
        .zip(target.into_iter())
        .map(|(l, r)| assert_eq!(l, r))
        .collect();
    Ok(())
}
