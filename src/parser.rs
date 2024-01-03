use std::io::Result;
use std::io::{Error, ErrorKind};

use crate::tokenizer::TokenKind;

use crate::error::new_error;

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

fn do_parsing(mut iterator: std::vec::IntoIter<TokenKind>) -> Result<ReturnNode> {
    match (
        iterator.next().ok_or_else(|| new_error("syntax error"))?,
        iterator.next().ok_or_else(|| new_error("syntax error"))?,
    ) {
        (crate::TokenKind::Return, crate::TokenKind::Int(value)) => {
            Ok(ReturnNode::new(parse_expression(value)))
        }
        _ => do_parsing(iterator),
    }
}

pub fn parse(tokenised_code: Vec<TokenKind>) -> ReturnNode {
    let iterator = tokenised_code.into_iter();
    match do_parsing(iterator) {
        Ok(return_node) => return_node,
        Err(e) => panic!("{}", e),
    }
}
