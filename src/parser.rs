use std::io::Result;

use crate::error::new_error;
use crate::tokenizer::TokenKind;

#[derive(Debug)]
pub enum Node {
    Expression(ExpressionNode),
    Return(ReturnNode),
}

#[derive(Debug)]
pub struct ExpressionNode {
    pub integer: String,
}

impl ExpressionNode {
    fn new(integer: String) -> Self {
        Self { integer: integer }
    }
}

#[derive(Debug)]
pub struct ReturnNode {
    pub expression: Box<Node>,
}

impl ReturnNode {
    fn new(expression: Node) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

fn parse_expression(value: String) -> Node {
    Node::Expression(ExpressionNode::new(value))
}

fn do_parsing(mut iterator: std::vec::IntoIter<TokenKind>) -> Result<Node> {
    match (
        iterator.next().ok_or_else(|| new_error("syntax error"))?,
        iterator.next().ok_or_else(|| new_error("syntax error"))?,
    ) {
        (crate::TokenKind::Return, crate::TokenKind::Int(value)) => {
            Ok(Node::Return(ReturnNode::new(parse_expression(value))))
        }
        _ => do_parsing(iterator),
    }
}

pub fn parse(tokenised_code: Vec<TokenKind>) -> Node {
    let iterator = tokenised_code.into_iter();
    match do_parsing(iterator) {
        Ok(return_node) => return_node,
        Err(e) => panic!("{}", e),
    }
}
