use std::io::Result;

use crate::error::new_error;
use crate::tokenizer::TokenKind;

use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub enum StatementNode {
    Exit(ExitNode),
    Assign(String, AssignNode),
    For(String, ExpressionNode),
    EndFor,
    While(ExpressionNode),
    EndWhile,
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
    Callable(String, Box<ExpressionNode>),
}

pub fn parse(line: Vec<TokenKind>) -> Result<StatementNode> {
    let iterator = line.into_iter();
    do_parsing(iterator)
}

fn do_parsing(mut iterator: IntoIter<TokenKind>) -> Result<StatementNode> {
    let current_token = iterator.next().ok_or_else(|| new_error("syntax error 1"))?;
    match current_token {
        TokenKind::Exit | TokenKind::VarName(_) | TokenKind::EndWhile | TokenKind::While => {
            parse_statement(current_token, iterator)
        }
        _ => Err(new_error("syntax error 2")),
    }
}

fn parse_statement(
    current_token: TokenKind,
    iterator: IntoIter<TokenKind>,
) -> Result<StatementNode> {
    match current_token {
        TokenKind::Exit => Ok(StatementNode::Exit(parse_exit(iterator)?)),
        TokenKind::VarName(name) => Ok(StatementNode::Assign(name, parse_assign(iterator)?)),
        TokenKind::For => parse_for(iterator),
        TokenKind::EndFor => Ok(StatementNode::EndFor),
        TokenKind::While => parse_while(iterator),
        TokenKind::EndWhile => Ok(StatementNode::EndWhile),
        _ => Err(new_error("syntax error 3")),
    }
}

fn parse_for(mut iterator: IntoIter<TokenKind>) -> Result<StatementNode> {
    let varname = match iterator.next().ok_or(new_error("syntax error 4"))? {
        TokenKind::VarName(name) => Ok(name),
        _ => Err(new_error("syntax error 5")),
    }?;
    match iterator.next().ok_or(new_error("syntax error 6"))? {
        TokenKind::In => Ok(()),
        _ => Err(new_error("syntax Error 7")),
    }?;
    let current_token = iterator.next().ok_or(new_error("syntax error 8"))?;
    Ok(StatementNode::For(
        varname,
        ExpressionParser::parse(iterator, current_token)?,
    ))
}

fn parse_while(mut iterator: IntoIter<TokenKind>) -> Result<StatementNode> {
    let exp_start = iterator.next().ok_or(new_error("invalid while"))?;
    Ok(StatementNode::While(ExpressionParser::parse(
        iterator, exp_start,
    )?))
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

// using a struct makes it easy to move the iterator between recursive calls
struct ExpressionParser {
    iterator: Peekable<IntoIter<TokenKind>>,
}

impl ExpressionParser {
    fn parse(iterator: IntoIter<TokenKind>, current_token: TokenKind) -> Result<ExpressionNode> {
        Self {
            iterator: iterator.peekable(),
        }
        .parse_expression(current_token, 1)
    }

    fn parse_expression_token(&mut self, token: TokenKind) -> Result<ExpressionNode> {
        match token {
            TokenKind::OpenParen => {
                let next_token = self.iterator.next().ok_or(new_error("syntax error"))?;
                //creates a bug because of precedance.
                self.parse_expression(next_token, 1)
            }
            TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
            TokenKind::VarName(name) => Ok(ExpressionNode::Var(name)),
            TokenKind::Callable(name) => {
                let next_token = self.iterator.next().ok_or(new_error("syntax error"))?;
                Ok(ExpressionNode::Callable(
                    name,
                    Box::new(self.parse_expression(next_token, 1)?),
                ))
            }
            _ => Err(new_error("syntax error: balse")),
        }
    }

    fn parse_expression(
        &mut self,
        current_token: TokenKind,
        current_precedence: u8,
    ) -> Result<ExpressionNode> {
        let mut expr = self.parse_expression_token(current_token);
        // too much indent lets refactor
        loop {
            let precedance = match self.iterator.peek() {
                Some(token) => {
                    let infix = match token {
                        TokenKind::Operator(infix) => Ok(infix.as_str()),
                        TokenKind::CloseParen => {
                            self.iterator.next();
                            break expr;
                        }
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
                None => break expr,
            };
            let op_token = self.iterator.next().unwrap();
            let infix = match op_token {
                TokenKind::Operator(infix) => Ok(infix),
                _ => Err(new_error("syntax error: invalid expression")),
            }?;
            let next_token = match self.iterator.next() {
                Some(token) => token,
                None => break expr,
            };
            let rh_expr = self.parse_expression(next_token, precedance);
            expr = Ok(Self::make_infix(expr?, rh_expr?, infix));
        }
    }

    fn make_infix(lh: ExpressionNode, rh: ExpressionNode, infix: String) -> ExpressionNode {
        ExpressionNode::Infix(Box::new(lh), infix.to_string(), Box::new(rh))
    }
}
