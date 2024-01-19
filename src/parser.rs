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
            let mut expr_parser = ExpressionParser::new(iterator);
            Ok(AssignNode::Expression(
                expr_parser.parse_expression(current_token)?,
            ))
        }
        _ => Err(new_error("Invalid Token")),
    }
}

fn parse_exit(mut iterator: IntoIter<TokenKind>) -> Result<ExitNode> {
    let err_msg = "syntax error: no exit value";
    let current_token = iterator.next().ok_or_else(|| new_error(err_msg))?;
    let mut expr_parser = ExpressionParser::new(iterator);
    Ok(ExitNode::Expression(
        expr_parser.parse_expression(current_token)?,
    ))
}

struct ExpressionParser {
    iterator: IntoIter<TokenKind>,
    current_precidence: u8,
}

impl ExpressionParser {
    fn new(iterator: IntoIter<TokenKind>) -> Self {
        Self {
            iterator: iterator,
            current_precidence: 1,
        }
    }

    fn parse_expression_token(token: TokenKind) -> Result<ExpressionNode> {
        match token {
            TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
            TokenKind::VarName(name) => Ok(ExpressionNode::Var(name)),
            _ => Err(new_error("syntax error: balse")),
        }
    }

    fn parse_expression(&mut self, current_token: TokenKind) -> Result<ExpressionNode> {
        let expr = match current_token {
            TokenKind::Int(_) => Self::parse_expression_token(current_token),
            TokenKind::VarName(_) => Self::parse_expression_token(current_token),
            _ => Err(new_error("syntax error: invalid expression")),
        };
        match self.iterator.next() {
            Some(next_token) => self.parse_expression_infix(next_token, expr),
            None => expr,
        }
    }

    fn parse_expression_infix(
        &mut self,
        current_token: TokenKind,
        mut expr: Result<ExpressionNode>,
    ) -> Result<ExpressionNode> {
        loop {
            let infix = match current_token {
                TokenKind::Operator(ref infix) => Ok(infix.as_str()),
                _ => Err(new_error("syntax error: invalid expression")),
            }?;
            let precidance: u8 = match infix {
                "+" | "-" => Ok(1),
                "*" | "/" => Ok(2),
                _ => Err(new_error("syntax error: unknown operator")),
            }?;
            if precidance < self.current_precidence {
                break expr;
            }
            //this next bit is written so weird
            let option_next_token = self.iterator.next();
            if let None = option_next_token {
                break expr;
            }
            let next_token = option_next_token.unwrap();
            //end weird
            self.current_precidence = precidance;
            dbg!(&infix);
            dbg!(&current_token);
            let rh_expr = self.parse_expression(next_token);
            expr = Ok(Self::make_infix(expr?, rh_expr?, infix.to_string()));
        }
    }

    fn make_infix(lh: ExpressionNode, rh: ExpressionNode, infix: String) -> ExpressionNode {
        ExpressionNode::Infix(Box::new(lh), infix.to_string(), Box::new(rh))
    }
}
