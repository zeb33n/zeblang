use std::io::{Error, Result};

use crate::tokenizer::TokenKind;

#[derive(Debug)]
struct ExpressionNode {
    integer: String,
}

impl ExpressionNode {
    fn new(integer: String) -> Self {
        Self { integer: integer }
    }
}

#[derive(Debug)]
pub struct ReturnNode {
    expression: ExpressionNode,
}

impl ReturnNode {
    fn new(expression: ExpressionNode) -> Self {
        Self {
            expression: expression,
        }
    }
}

fn parse_expression(value: String) -> ExpressionNode {
    ExpressionNode::new(value)
}

pub fn parse(tokenised_code: Vec<TokenKind>) -> Option<ReturnNode> {
    let mut iterator = tokenised_code.into_iter();
    match (iterator.next().unwrap(), iterator.next().unwrap()) {
        (crate::TokenKind::Return, crate::TokenKind::Int(value)) => {
            Some(ReturnNode::new(parse_expression(value)))
        }
        _ => None,
    }
}
