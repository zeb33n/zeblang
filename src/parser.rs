use std::io::Result;

use crate::error::new_error;
use crate::tokenizer::TokenKind;

use std::iter::Peekable;
use std::vec::IntoIter;

// this file has become a bit spaghetti and could do with a refactor

#[derive(Debug)]
pub enum StatementNode {
    Exit(ExpressionNode),
    Assign(String, ExpressionNode),
    AssignIndex(String, ExpressionNode, ExpressionNode),
    For(String, ExpressionNode),
    EndFor,
    If(ExpressionNode),
    EndIf,
    While(ExpressionNode),
    EndWhile,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Value(String),
    Var(String),
    Index(String, Box<ExpressionNode>),
    Callable(String, Box<ExpressionNode>),
    Infix(Box<ExpressionNode>, String, Box<ExpressionNode>),
    Array(Vec<Box<ExpressionNode>>),
    PreAllocArray(usize),
}

pub fn parse(line: Vec<TokenKind>) -> Result<StatementNode> {
    let iterator = line.into_iter();
    Parser::parse(iterator)
}

struct Parser {
    iterator: Peekable<IntoIter<TokenKind>>,
}

impl Parser {
    fn parse(iterator: IntoIter<TokenKind>) -> Result<StatementNode> {
        Self {
            iterator: iterator.peekable(),
        }
        .do_parsing()
    }

    fn do_parsing(&mut self) -> Result<StatementNode> {
        let current_token = self
            .iterator
            .next()
            .ok_or_else(|| new_error("syntax error 1"))?;
        self.parse_statement(current_token)
    }

    fn parse_statement(&mut self, current_token: TokenKind) -> Result<StatementNode> {
        match current_token {
            TokenKind::Exit => Ok(self.parse_exit()?),
            TokenKind::VarName(name) => Ok(self.parse_assign(name)?),
            TokenKind::For => self.parse_for(),
            TokenKind::EndFor => Ok(StatementNode::EndFor),
            TokenKind::While => self.parse_while(),
            TokenKind::EndWhile => Ok(StatementNode::EndWhile),
            TokenKind::If => self.parse_if(),
            TokenKind::EndIf => Ok(StatementNode::EndIf),
            _ => Err(new_error("syntax error 3")),
        }
    }

    fn parse_for(&mut self) -> Result<StatementNode> {
        let varname = match self.iterator.next().ok_or(new_error("syntax error 4"))? {
            TokenKind::VarName(name) => Ok(name),
            _ => Err(new_error("syntax error 5")),
        }?;
        match self.iterator.next().ok_or(new_error("syntax error 6"))? {
            TokenKind::In => Ok(()),
            _ => Err(new_error("syntax Error 7")),
        }?;
        let current_token = self.iterator.next().ok_or(new_error("syntax error 8"))?;
        Ok(StatementNode::For(
            varname,
            self.parse_expression(current_token, 1)?,
        ))
    }

    fn parse_if(&mut self) -> Result<StatementNode> {
        let exp_start = self.iterator.next().ok_or(new_error("invalid if"))?;
        Ok(StatementNode::If(self.parse_expression(exp_start, 1)?))
    }

    fn parse_while(&mut self) -> Result<StatementNode> {
        let exp_start = self.iterator.next().ok_or(new_error("invalid while"))?;
        Ok(StatementNode::While(self.parse_expression(exp_start, 1)?))
    }

    fn parse_assign(&mut self, name: String) -> Result<StatementNode> {
        let current_token = self
            .iterator
            .next()
            .ok_or_else(|| new_error("syntax error: no equals"))?;
        match current_token {
            TokenKind::Assign => {
                dbg!(&current_token);
                let current_token = self
                    .iterator
                    .next()
                    .ok_or_else(|| new_error("syntax error: no equals"))?;
                Ok(StatementNode::Assign(
                    name,
                    self.parse_expression(current_token, 1)?,
                ))
            }
            TokenKind::OpenSquare => {
                let current_token = self
                    .iterator
                    .next()
                    .ok_or_else(|| new_error("syntax error: no equals"))?;
                let index_expr = self.parse_expression(current_token, 1)?;
                self.iterator.next();
                self.iterator.next();
                let current_token = self
                    .iterator
                    .next()
                    .ok_or_else(|| new_error("syntax error: expected = and expression"))?;
                let assign_expr = self.parse_expression(current_token, 1)?;
                Ok(StatementNode::AssignIndex(name, index_expr, assign_expr))
            }
            _ => Err(new_error("Invalid Token")),
        }
    }

    fn parse_exit(&mut self) -> Result<StatementNode> {
        let err_msg = "syntax error: no exit value";
        let current_token = self.iterator.next().ok_or_else(|| new_error(err_msg))?;
        Ok(StatementNode::Exit(
            self.parse_expression(current_token, 1)?,
        ))
    }

    fn parse_expression(
        &mut self,
        current_token: TokenKind,
        current_precedence: u8,
    ) -> Result<ExpressionNode> {
        let mut expr = self.parse_expression_token(current_token);
        loop {
            let infix = match self.get_infix_op()? {
                Some(infix) => infix,
                None => break expr,
            };
            let precedance = self.get_precedance(infix)?;
            if precedance < current_precedence {
                break expr;
            }
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

    fn parse_expression_token(&mut self, token: TokenKind) -> Result<ExpressionNode> {
        match token {
            TokenKind::OpenParen => {
                let next_token = self.iterator.next().ok_or(new_error("syntax error 3"))?;
                self.parse_expression(next_token, 1)
            }
            TokenKind::OpenSquare => self.parse_array(),
            TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
            TokenKind::VarName(name) => self.parse_var(name),
            TokenKind::Callable(name) => {
                let next_token = self.iterator.next().ok_or(new_error("syntax error 2"))?;
                Ok(ExpressionNode::Callable(
                    name,
                    Box::new(self.parse_expression(next_token, 1)?),
                ))
            }
            _ => Err(new_error("syntax error: balse")),
        }
    }

    fn parse_var(&mut self, name: String) -> Result<ExpressionNode> {
        match self.iterator.peek() {
            Some(token) if token == &TokenKind::OpenSquare => {
                self.iterator.next();
                let next = self
                    .iterator
                    .next()
                    .ok_or(new_error("expected expression"))?;
                let out = Ok(ExpressionNode::Index(
                    name,
                    Box::new(self.parse_expression(next, 1)?),
                ));
                match self.iterator.next() {
                    Some(token) if token == TokenKind::CloseSquare => Ok(()),
                    _ => Err(new_error("expected ]")),
                }?;
                out
            }
            _ => Ok(ExpressionNode::Var(name)),
        }
    }

    fn parse_array(&mut self) -> Result<ExpressionNode> {
        let mut out: Vec<Box<ExpressionNode>> = Vec::new();
        loop {
            let next_token = self.iterator.next().ok_or(new_error("syntax error 1"))?;
            match next_token {
                TokenKind::Comma => continue,
                TokenKind::CloseSquare => break Ok(ExpressionNode::Array(out)),
                TokenKind::Size => {
                    break match self.iterator.next().ok_or(new_error("expected size"))? {
                        TokenKind::Int(value) => {
                            Ok(ExpressionNode::PreAllocArray(value.parse().unwrap()))
                        }
                        _ => Err(new_error("not a valid size")),
                    };
                }
                _ => out.push(Box::new(self.parse_expression(next_token, 1)?)),
            }
        }
    }

    fn get_infix_op(&mut self) -> Result<Option<String>> {
        match self.iterator.peek() {
            Some(token) => {
                let infix = match token {
                    TokenKind::Operator(infix) => Ok(Some(infix)),
                    TokenKind::CloseParen => {
                        self.iterator.next();
                        Ok(None)
                    }
                    TokenKind::CloseSquare | TokenKind::Comma => Ok(None),
                    _ => Err(new_error("syntax error: expected operator")),
                }?;
                Ok(infix.cloned()) //do a better job here
            }
            None => Ok(None),
        }
    }

    fn get_precedance(&mut self, infix: String) -> Result<u8> {
        match infix.as_str() {
            "==" | "!=" => Ok(1),
            "+" | "-" => Ok(2),
            "*" | "/" | "%" => Ok(3),
            _ => Err(new_error(
                format!("syntax error: unknown operator {}", infix).as_str(),
            )),
        }
    }

    fn make_infix(lh: ExpressionNode, rh: ExpressionNode, infix: String) -> ExpressionNode {
        ExpressionNode::Infix(Box::new(lh), infix.to_string(), Box::new(rh))
    }
}
