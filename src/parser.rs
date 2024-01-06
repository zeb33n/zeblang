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
    Var(String),
    Infix(Box<ExpressionNode>, String, Box<ExpressionNode>),
}

pub fn parse(line: Vec<TokenKind>) -> Result<StatementNode> {
    let iterator = line.into_iter();
    do_parsing(iterator)
}

fn do_parsing(mut iterator: IntoIter<TokenKind>) -> Result<StatementNode> {
    let current_token = iterator.next().ok_or_else(|| new_error("syntax error"))?;
    match current_token {
        crate::TokenKind::Exit | crate::TokenKind::VarName(_) => {
            parse_statement(current_token, iterator)
        }
        _ => Err(new_error("syntax error")),
    }
}

fn parse_statement(
    current_token: TokenKind,
    iterator: IntoIter<TokenKind>,
) -> Result<StatementNode> {
    match current_token {
        TokenKind::Exit => Ok(StatementNode::Exit(parse_exit(iterator)?)),
        TokenKind::VarName(name) => Ok(StatementNode::Assign(name, parse_assign(iterator)?)),
        _ => Err(new_error("syntax error:")),
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
            Ok(AssignNode::Expression(parse_expression(
                current_token,
                iterator,
            )?))
        }
        _ => Err(new_error("Invalid Token")),
    }
}

fn parse_exit(mut iterator: IntoIter<TokenKind>) -> Result<ExitNode> {
    let err_msg = "syntax error: no exit value";
    let current_token = iterator.next().ok_or_else(|| new_error(err_msg))?;
    Ok(ExitNode::Expression(parse_expression(
        current_token,
        iterator,
    )?))
}

fn parse_expression(
    current_token: TokenKind,
    iterator: IntoIter<TokenKind>,
) -> Result<ExpressionNode> {
    match current_token {
        TokenKind::Int(_) => do_parse_expression(current_token, iterator),
        TokenKind::VarName(_) => do_parse_expression(current_token, iterator),
        _ => Err(new_error("syntax error: invalid expression")),
    }
}

//this can be better abstract out the different cases and parse them individualy?
fn do_parse_expression(
    current_token: TokenKind,
    mut iterator: IntoIter<TokenKind>,
) -> Result<ExpressionNode> {
    let current_node = match current_token {
        TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
        TokenKind::VarName(name) => Ok(ExpressionNode::Var(name)),
        _ => Err(new_error("syntax error: balse")),
    }?;

    match iterator.next() {
        Some(next_token) => {
            // learn if let
            let infix = match next_token {
                TokenKind::Operator(infix) => Ok(infix),
                _ => Err(new_error("syntax error: balse")),
            }?;
            let expression = parse_expression(
                iterator
                    .next()
                    .ok_or(new_error("syntax error: wrong use of infix"))?,
                iterator,
            )?;
            //
            // this wont work for operator presidance
            //
            // 1 - (2 * 3) + (4 / 5)
            //
            // should transforms into:
            //
            // expr(
            //     expr(
            //         1
            //         -
            //         expr(
            //             2
            //             *
            //             3
            //         )
            //     +
            //     expr(
            //         4
            //         /
            //         5
            // )
            //
            //
            // Infix(expr, op, expr) -> this works as data structure
            //
            // when you multiply or divide you look for another
            // multiply or divide if it doesnt exist you kill that branch
            // and go back to the last plus or minus. I think its like a
            // nested version of what I already have. Rust wont let me
            // miss any edge cases :)
            //
            Ok(ExpressionNode::Infix(
                Box::new(current_node),
                infix,
                Box::new(expression),
            ))
        }
        None => Ok(current_node),
    }
}

// where do i get T from
//fn syntax_error() -> Result<T> {
//Err(new_error("syntax error: balse"))
//}
