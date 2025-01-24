use std::collections::HashMap;
use std::slice::Iter;

use crate::parser::{ExpressionNode, StatementNode};

use crate::printing::zeblang_print;

#[derive(Clone)]
enum Variable {
    Int(i32),
    Array(Vec<Box<Variable>>),
}

impl Variable {
    fn to_string(&self) -> String {
        match self {
            &Self::Int(i) => i.to_string(),
            Self::Array(v) => v
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

struct Interpreter<'a> {
    vars: &'a mut HashMap<String, Variable>,
    iter: Iter<'a, StatementNode>,
    out: Result<i32, String>,
}

pub fn interpret(parse_tree: Vec<StatementNode>) -> Result<i32, String> {
    return Interpreter {
        vars: &mut HashMap::new(),
        iter: parse_tree.iter(),
        out: Ok(0),
    }
    .run();
}

fn interpret_with_context(
    parse_tree: Iter<'_, StatementNode>,
    context: &'_ mut HashMap<String, Variable>,
) -> Result<i32, String> {
    return Interpreter {
        vars: context,
        iter: parse_tree,
        out: Ok(0),
    }
    .run();
}

impl<'a> Interpreter<'a> {
    fn run(&mut self) -> Result<i32, String> {
        let mut statement_option = self.iter.next();
        while statement_option.is_some() {
            let statement = statement_option.ok_or("Some Looping went wrong!")?;
            self.interpret_statement(statement)?;
            statement_option = self.iter.next();
        }
        return self.out.to_owned();
    }

    fn interpret_statement(&mut self, statement: &StatementNode) -> Result<(), String> {
        match statement {
            // TODO exit is currently more of a return. just exits current context
            StatementNode::Exit(node) => {
                self.out = match self.interpret_exit(node)? {
                    Variable::Int(i) => Ok(i),
                    _ => Err("wrong type passed to Exit".to_string()),
                };
                return Ok(());
            }
            StatementNode::Assign(name, node) => {
                let value = self.interpret_expr(node)?;
                self.vars.insert(name.to_owned(), value);
            }
            StatementNode::While(node) => self.interpret_while(node)?,
            StatementNode::If(node) => self.interpret_if(node)?,
            _ => todo!(),
        }
        Ok(())
    }

    // TODO Clean up repeated code. Functions and for loops will also likely be similar
    fn interpret_if(&mut self, node: &ExpressionNode) -> Result<(), String> {
        let mut nests = 1;
        let mut stmnts: Vec<StatementNode> = Vec::new();
        while nests != 0 {
            let next = self.iter.next().ok_or("While loop not closed")?;
            match next {
                &StatementNode::EndIf => nests -= 1,
                &StatementNode::If(_) => nests += 1,
                _ => (),
            }
            stmnts.push(next.to_owned());
        }
        stmnts.pop(); // remove last endif
        if match self.interpret_expr(node)? {
            Variable::Int(i) => i,
            _ => return Err("value in if has no truthiness".to_string()),
        } != 0
        {
            interpret_with_context(stmnts.iter(), self.vars)?;
        }
        Ok(())
    }

    fn interpret_while(&mut self, node: &ExpressionNode) -> Result<(), String> {
        let mut nests = 1;
        let mut stmnts: Vec<StatementNode> = Vec::new();
        while nests != 0 {
            let next = self.iter.next().ok_or("While loop not closed")?;
            match next {
                &StatementNode::EndWhile => nests -= 1,
                &StatementNode::While(_) => nests += 1,
                _ => (),
            }
            stmnts.push(next.to_owned());
        }
        stmnts.pop(); // remove last endwhile
        while match self.interpret_expr(node)? {
            Variable::Int(i) => i,
            _ => return Err("value in while has no truthiness".to_string()),
        } != 0
        {
            interpret_with_context(stmnts.iter(), self.vars)?;
        }
        Ok(())
    }

    fn interpret_exit(&mut self, node: &ExpressionNode) -> Result<Variable, String> {
        let value = self.interpret_expr(node);
        return value;
    }

    fn interpret_expr(&mut self, node: &ExpressionNode) -> Result<Variable, String> {
        return match node {
            ExpressionNode::Int(value) => Ok(Variable::Int(
                value.parse::<i32>().ok().ok_or("invalid int".to_string())?,
            )),
            // TODO This will be awful for massive arrays
            ExpressionNode::Var(name) => Ok(self.vars.get(name).ok_or("Undefined var")?.to_owned()),
            ExpressionNode::Infix(node1, infix, node2) => {
                let v1 = self.interpret_expr(node1)?;
                let v2 = self.interpret_expr(node2)?;

                let (i1, i2) = match (v1, v2) {
                    (Variable::Int(i1), Variable::Int(i2)) => (i1, i2),
                    _ => return Err("can only add integers".to_string()),
                };

                match infix.as_str() {
                    "+" => Ok(Variable::Int(i1 + i2)),
                    "-" => Ok(Variable::Int(i1 - i2)),
                    "*" => Ok(Variable::Int(i1 * i2)),
                    "/" => Ok(Variable::Int(i1 / i2)),
                    "==" => Ok(Variable::Int((i1 == i2) as i32)),
                    "!=" => Ok(Variable::Int((i1 != i2) as i32)),
                    "%" => Ok(Variable::Int(i1 % i2)),
                    _ => Err("Invalid Infix op".to_string()),
                }
            }
            ExpressionNode::Callable(name, nodes) => match name.as_str() {
                "print" => self.interpret_print(nodes),
                _ => todo!(),
            },
            _ => todo!(),
        };
    }

    fn interpret_print(&mut self, nodes: &Vec<Box<ExpressionNode>>) -> Result<Variable, String> {
        if nodes.len() > 1 {
            return Err("too many arguments to print".to_string());
        };
        let node = nodes.get(0).ok_or("No argument provided")?;
        let out = self.interpret_expr(&*node)?;
        zeblang_print(&out.to_string());
        return Ok(out);
    }
}
