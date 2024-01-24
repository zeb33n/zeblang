use std::io::Result;

use crate::error::new_error;
use crate::tokenizer::TokenKind;

use std::iter::Peekable;
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
            Ok(AssignNode::Expression(ExpressionParser::parse(
                iterator,
                current_token,
            )?))
        }
        _ => Err(new_error("Invalid Token")),
    }
}

fn parse_exit(mut iterator: IntoIter<TokenKind>) -> Result<ExitNode> {
    let err_msg = "syntax error: no exit value";
    let current_token = iterator.next().ok_or_else(|| new_error(err_msg))?;
    Ok(ExitNode::Expression(ExpressionParser::parse(
        iterator,
        current_token,
    )?))
}

struct ExpressionParser {
    // using a struct makes it easy to move the iterator between recursive calls
    iterator: Peekable<IntoIter<TokenKind>>,
}

impl ExpressionParser {
    fn parse(iterator: IntoIter<TokenKind>, current_token: TokenKind) -> Result<ExpressionNode> {
        Self {
            iterator: iterator.peekable(),
        }
        .parse_expression(current_token, 1)
    }

    fn parse_expression_token(token: TokenKind) -> Result<ExpressionNode> {
        match token {
            TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
            TokenKind::VarName(name) => Ok(ExpressionNode::Var(name)),
            _ => Err(new_error("syntax error: balse")),
        }
    }

    fn parse_expression(
        &mut self,
        current_token: TokenKind,
        current_precedence: u8,
    ) -> Result<ExpressionNode> {
        let mut expr = match current_token {
            TokenKind::Int(_) => Self::parse_expression_token(current_token),
            TokenKind::VarName(_) => Self::parse_expression_token(current_token),
            _ => Err(new_error("syntax error: invalid expression")),
        };
        loop {
            let precedance = match self.iterator.peek() {
                Some(token) => {
                    let infix = match token {
                        TokenKind::Operator(infix) => Ok(infix.as_str()),
                        _ => Err(new_error("syntax error: expected operator")),
                    }?;
                    let precedence: u8 = match infix {
                        "+" | "-" => Ok(1),
                        "*" | "/" => Ok(2),
                        _ => Err(new_error(
                            format!("syntax error: unknown operator {}", infix).as_str(),
                        )),
                    }?;
                    if precedence < current_precedence {
                        break expr;
                    }
                    precedence
                }
                None => {
                    break expr;
                }
            };
            let op_token = self.iterator.next().unwrap();
            let infix = match op_token {
                TokenKind::Operator(infix) => Ok(infix),
                _ => Err(new_error("syntax error: invalid expression")),
            }?;
            let next_token = match self.iterator.next() {
                Some(token) => token,
                None => {
                    break expr;
                }
            };
            let rh_expr = self.parse_expression(next_token, precedance);
            expr = Ok(Self::make_infix(expr?, rh_expr?, infix));
        }
    }

    fn make_infix(lh: ExpressionNode, rh: ExpressionNode, infix: String) -> ExpressionNode {
        ExpressionNode::Infix(Box::new(lh), infix.to_string(), Box::new(rh))
    }
}
