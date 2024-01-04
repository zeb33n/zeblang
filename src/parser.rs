use std::io::Result;

use crate::error::new_error;
use crate::tokenizer::TokenKind;

use std::vec::IntoIter;

#[derive(Debug)]
pub enum StatementNode {
    Exit(ExitNode),
    Assign(String, AssignNode),
}

#[derive(Debug)]
pub enum ExitNode {
    Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum AssignNode {
    Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum ExpressionNode {
    Value(String),
}

pub fn parse(tokenised_code: Vec<TokenKind>) -> StatementNode {
    let iterator = tokenised_code.into_iter();
    match do_parsing(iterator) {
        Ok(return_node) => return_node,
        Err(e) => panic!("{}", e),
    }
}

// this only makes one statement per program. we should return a vector of statements
fn do_parsing(mut iterator: IntoIter<TokenKind>) -> Result<StatementNode> {
    let current_token = iterator.next().ok_or_else(|| new_error("syntax error"))?;
    match current_token {
        crate::TokenKind::Exit | crate::TokenKind::VarName(_) => {
            parse_statement(current_token, iterator)
        }
        _ => do_parsing(iterator),
    }
}

fn parse_statement(
    current_token: TokenKind,
    iterator: IntoIter<TokenKind>,
) -> Result<StatementNode> {
    match current_token {
        TokenKind::Exit => Ok(StatementNode::Exit(parse_exit(iterator)?)),
        TokenKind::VarName(ref name) => Ok(StatementNode::Assign(
            name.to_owned(),
            parse_assign(iterator)?,
        )),
        _ => Err(new_error("syntax error")),
    }
}

fn parse_assign(mut iterator: IntoIter<TokenKind>) -> Result<AssignNode> {
    let current_token = iterator
        .next()
        .ok_or_else(|| new_error("syntax error: no equals"))?;
    match current_token {
        TokenKind::Assign => {
            let current_token = iterator
                .next()
                .ok_or_else(|| new_error("syntax error: no equals"))?;
            Ok(AssignNode::Expression(parse_expression(current_token)?))
        }
        _ => Err(new_error("Invalid Token")),
    }
}

fn parse_exit(mut iterator: IntoIter<TokenKind>) -> Result<ExitNode> {
    let err_msg = "syntax error: no exit value";
    let current_token = iterator.next().ok_or_else(|| new_error(err_msg))?;
    Ok(ExitNode::Expression(parse_expression(current_token)?))
}

fn parse_expression(current_token: TokenKind) -> Result<ExpressionNode> {
    match current_token {
        TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
        _ => Err(new_error("syntax error: invalid expression")),
    }
}

//iterator.peek().ok_or_else(|| new_error("syntax error"))?,
