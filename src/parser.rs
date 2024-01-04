use std::io::Result;

use crate::error::new_error;
use crate::tokenizer::TokenKind;

#[derive(Debug)]
pub enum StatementNode {
    Exit(ExitNode),
    Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum ExitNode {
    Expression(ExpressionNode),
}

#[derive(Debug)]
pub enum ExpressionNode {
    Value(String),
}

fn parse_expression(value: String) -> ExpressionNode {
    ExpressionNode::Value(value)
}

fn do_parsing(mut iterator: std::vec::IntoIter<TokenKind>) -> Result<StatementNode> {
    match (
        iterator.next().ok_or_else(|| new_error("syntax error"))?,
        iterator.next().ok_or_else(|| new_error("syntax error"))?,
    ) {
        (crate::TokenKind::Exit, crate::TokenKind::Int(value)) => Ok(StatementNode::Exit(
            ExitNode::Expression(parse_expression(value)),
        )),
        _ => do_parsing(iterator),
    }
}

pub fn parse(tokenised_code: Vec<TokenKind>) -> StatementNode {
    let iterator = tokenised_code.into_iter();
    match do_parsing(iterator) {
        Ok(return_node) => return_node,
        Err(e) => panic!("{}", e),
    }
}

//pub enum Node {
//Expression(ExpressionNode),
//Exit(ExitNode),
//Statement(StatementNode),
//}

//#[derive(Debug)]
//pub struct StatementNode {
//pub exit: Box<Node>,
//pub expression: Box<Node>
//}

//#[derive(Debug)]
//pub struct ExitNode {
//pub expression: Box<Node>,
//}

//#[derive(Debug)]
//pub struct ExpressionNode {
//pub integer: String,
//}

//impl ExpressionNode {
//fn new(integer: String) -> Self {
//Self { integer: integer }
//}
//}

//impl ExitNode {
//fn new(expression: Node) -> Self {
//Self {
//expression: Box::new(expression),
//}
//}
//}
