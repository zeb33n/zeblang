use crate::tokenizer::{Lexer, TokenKind};
use std::io::Result;

#[test]
fn test_lexer() -> Result<()> {
    let out =
        Lexer::lex(" =;()*0123 789 exit rof exit_-z_A01+foo(1+1)==!=->:int:AzZ _".to_string())?;
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
        TokenKind::Callable("foo".to_string()),
        TokenKind::Int("1".to_string()),
        TokenKind::Operator("+".to_string()),
        TokenKind::Int("1".to_string()),
        TokenKind::CloseParen,
        TokenKind::Operator("==".to_string()),
        TokenKind::Operator("!=".to_string()),
        TokenKind::Range,
        TokenKind::Type("int".to_string()),
        TokenKind::Type("AzZ".to_string()),
        TokenKind::VarName("_".to_string()),
    ];
    assert_eq!(&target, &out);
    let _: Vec<_> = out
        .into_iter()
        .zip(target.into_iter())
        .map(|(l, r)| assert_eq!(l, r))
        .collect();
    Ok(())
}

#[test]
fn test_func_lex() -> Result<()> {
    let out = Lexer::lex("foo blah(a, bee) return a oof".to_string())?;
    let target = vec![
        TokenKind::Func,
        TokenKind::Callable("blah".to_string()),
        TokenKind::VarName("a".to_string()),
        TokenKind::Comma,
        TokenKind::VarName("bee".to_string()),
        TokenKind::CloseParen,
        TokenKind::Return,
        TokenKind::VarName("a".to_string()),
        TokenKind::EndFunc,
    ];
    assert_eq!(&target, &out);
    Ok(())
}
