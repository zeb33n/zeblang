use std::io::Result;

use crate::error::syntax_error;
use crate::tokenizer::TokenKind;

use std::iter::Peekable;
use std::vec::IntoIter;

use serde::Serialize;

#[derive(Debug, Serialize)]
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
    Func(String, Vec<String>),
    EndFunc,
}

#[derive(Debug, Serialize)]
pub enum ExpressionNode {
    Value(String),
    Var(String),
    Index(String, Box<ExpressionNode>),
    Callable(String, Vec<Box<ExpressionNode>>),
    Infix(Box<ExpressionNode>, String, Box<ExpressionNode>),
    Array(Vec<Box<ExpressionNode>>),
    PreAllocArray(usize),
}

pub fn parse(line: Vec<TokenKind>, line_num: usize) -> Result<StatementNode> {
    let iterator = line.into_iter();
    Parser::parse(iterator, line_num)
}

struct Parser {
    iterator: Peekable<IntoIter<TokenKind>>,
    line: usize,
}

impl Parser {
    fn parse(iterator: IntoIter<TokenKind>, line_num: usize) -> Result<StatementNode> {
        Self {
            iterator: iterator.peekable(),
            line: line_num,
        }
        .do_parsing()
    }

    fn do_parsing(&mut self) -> Result<StatementNode> {
        let current_token = self
            .iterator
            .next()
            .ok_or(syntax_error("no tokens found", self.line))?;
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
            TokenKind::Func => self.parse_func_dec(),
            TokenKind::EndFunc => Ok(StatementNode::EndFunc),
            _ => Err(syntax_error("not a valid line start", self.line)),
        }
    }

    fn parse_for(&mut self) -> Result<StatementNode> {
        let varname = match self
            .iterator
            .next()
            .ok_or(syntax_error("expected var name", self.line))?
        {
            TokenKind::VarName(name) => Ok(name),
            _ => Err(syntax_error("not a valid varname", self.line)),
        }?;
        match self
            .iterator
            .next()
            .ok_or(syntax_error("expected in", self.line))?
        {
            TokenKind::In => Ok(()),
            _ => Err(syntax_error("expected in", self.line)),
        }?;
        let current_token = self
            .iterator
            .next()
            .ok_or(syntax_error("expected arraylike", self.line))?;
        Ok(StatementNode::For(
            varname,
            self.parse_expression(current_token, 1)?,
        ))
    }

    fn parse_if(&mut self) -> Result<StatementNode> {
        let exp_start = self
            .iterator
            .next()
            .ok_or(syntax_error("expected expression", self.line))?;
        Ok(StatementNode::If(self.parse_expression(exp_start, 1)?))
    }

    fn parse_while(&mut self) -> Result<StatementNode> {
        let exp_start = self
            .iterator
            .next()
            .ok_or(syntax_error("expected expression", self.line))?;
        Ok(StatementNode::While(self.parse_expression(exp_start, 1)?))
    }

    fn parse_func_dec(&mut self) -> Result<StatementNode> {
        if let Some(TokenKind::Callable(name)) = self.iterator.next() {
            let mut args: Vec<String> = Vec::new();
            loop {
                match self
                    .iterator
                    .next()
                    .ok_or(syntax_error("expected )", self.line))?
                {
                    TokenKind::VarName(vname) => args.push(vname),
                    TokenKind::Comma => continue,
                    TokenKind::CloseParen => return Ok(StatementNode::Func(name, args)),
                    _ => return Err(syntax_error("unexpected token", self.line)),
                };
            }
        }
        return Err(syntax_error("expected function name", self.line));
    }

    fn parse_assign(&mut self, name: String) -> Result<StatementNode> {
        let current_token = self
            .iterator
            .next()
            .ok_or_else(|| syntax_error("expected =", self.line))?;
        match current_token {
            TokenKind::Assign => {
                let current_token = self
                    .iterator
                    .next()
                    .ok_or_else(|| syntax_error("expected expression", self.line))?;
                Ok(StatementNode::Assign(
                    name,
                    self.parse_expression(current_token, 1)?,
                ))
            }
            TokenKind::OpenSquare => {
                let current_token = self
                    .iterator
                    .next()
                    .ok_or_else(|| syntax_error("expected expression", self.line))?;
                let index_expr = self.parse_expression(current_token, 1)?;
                self.iterator.next();
                self.iterator.next();
                let current_token = self
                    .iterator
                    .next()
                    .ok_or_else(|| syntax_error("expected = and expression", self.line))?;
                let assign_expr = self.parse_expression(current_token, 1)?;
                Ok(StatementNode::AssignIndex(name, index_expr, assign_expr))
            }
            _ => Err(syntax_error("Invalid Token", self.line)),
        }
    }

    fn parse_exit(&mut self) -> Result<StatementNode> {
        let err_msg = "expected expression";
        let current_token = self
            .iterator
            .next()
            .ok_or_else(|| syntax_error(err_msg, self.line))?;
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
                _ => Err(syntax_error("invalid infix op", self.line)),
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
                let next_token = self
                    .iterator
                    .next()
                    .ok_or(syntax_error("expected expression", self.line))?;
                self.parse_expression(next_token, 1)
            }
            TokenKind::OpenSquare => self.parse_array(),
            TokenKind::Int(value) => Ok(ExpressionNode::Value(value)),
            TokenKind::VarName(name) => self.parse_var(name),
            TokenKind::Callable(name) => self.parse_callable(name),
            _ => Err(syntax_error("invalid expression", self.line)),
        }
    }

    //bug in here somewhere
    fn parse_callable(&mut self, name: String) -> Result<ExpressionNode> {
        let mut out: Vec<Box<ExpressionNode>> = Vec::new();
        loop {
            dbg!(self.iterator.peek());
            let next_token = match self.iterator.next() {
                Some(TokenKind::CloseParen) | None => {
                    break Ok(ExpressionNode::Callable(name, out))
                }
                Some(token) => token,
            };
            match next_token {
                TokenKind::Comma => continue,
                // close paren never arrives coz its part of an expression
                // TokenKind::CloseParen => break Ok(ExpressionNode::Callable(name, out)),
                _ => out.push(Box::new(self.parse_expression(next_token, 1)?)),
            }
        }
    }

    fn parse_var(&mut self, name: String) -> Result<ExpressionNode> {
        match self.iterator.peek() {
            Some(token) if token == &TokenKind::OpenSquare => {
                self.iterator.next();
                let next = self
                    .iterator
                    .next()
                    .ok_or(syntax_error("expected expression", self.line))?;
                let out = Ok(ExpressionNode::Index(
                    name,
                    Box::new(self.parse_expression(next, 1)?),
                ));
                match self.iterator.next() {
                    Some(token) if token == TokenKind::CloseSquare => Ok(()),
                    _ => Err(syntax_error("expected ]", self.line)),
                }?;
                out
            }
            _ => Ok(ExpressionNode::Var(name)),
        }
    }

    fn parse_array(&mut self) -> Result<ExpressionNode> {
        let mut out: Vec<Box<ExpressionNode>> = Vec::new();
        loop {
            let next_token = self
                .iterator
                .next()
                .ok_or(syntax_error("expected expression", self.line))?;
            match next_token {
                TokenKind::Comma => continue,
                TokenKind::CloseSquare => break Ok(ExpressionNode::Array(out)),
                TokenKind::Size => {
                    break match self
                        .iterator
                        .next()
                        .ok_or(syntax_error("expected size", self.line))?
                    {
                        TokenKind::Int(value) => {
                            Ok(ExpressionNode::PreAllocArray(value.parse().unwrap()))
                        }
                        _ => Err(syntax_error("not a valid size", self.line)),
                    };
                }
                _ => out.push(Box::new(self.parse_expression(next_token, 1)?)),
            }
        }
    }

    // close paren works liek this since it checks next space for an op
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
                    _ => Err(syntax_error("expected operator", self.line)),
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
            _ => Err(syntax_error(
                format!("unknown operator {}", infix).as_str(),
                self.line,
            )),
        }
    }

    fn make_infix(lh: ExpressionNode, rh: ExpressionNode, infix: String) -> ExpressionNode {
        ExpressionNode::Infix(Box::new(lh), infix.to_string(), Box::new(rh))
    }
}
