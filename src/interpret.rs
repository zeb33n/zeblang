use std::cell::RefCell;
use std::collections::HashMap;
use std::slice::Iter;

use crate::parser::{ExpressionNode, StatementNode};

use crate::printing::zeblang_print;

#[derive(Clone, Debug)]
enum Variable {
    Int(i32),
    Array(RefCell<Vec<Variable>>),
}

// TODO expand this to refactor -> methods for infix etc
impl Variable {
    fn to_string(&self) -> String {
        match self {
            &Self::Int(i) => i.to_string(),
            Self::Array(v) => v
                .borrow()
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(","),
        }
    }
}

type ZebFunc = (Vec<String>, Vec<StatementNode>);

struct Interpreter<'a> {
    vars: &'a mut HashMap<String, Variable>,
    iter: Iter<'a, StatementNode>,
    out: (Variable, bool),
    funcs: &'a mut HashMap<String, ZebFunc>,
}

pub fn interpret(parse_tree: Vec<StatementNode>) -> Result<i32, String> {
    let (out, _) = Interpreter {
        vars: &mut HashMap::new(),
        iter: parse_tree.iter(),
        out: (Variable::Int(0), false),
        funcs: &mut HashMap::new(),
    }
    .run()?;
    return match out {
        Variable::Int(i) => Ok(i),
        _ => Err("out is not int".to_string()),
    };
}

fn interpret_with_context(
    parse_tree: Iter<'_, StatementNode>,
    context: &'_ mut HashMap<String, Variable>,
    funcs: &'_ mut HashMap<String, ZebFunc>,
) -> Result<(Variable, bool), String> {
    return Interpreter {
        vars: context,
        iter: parse_tree,
        out: (Variable::Int(0), false),
        funcs,
    }
    .run();
}

//TODO how to break when return is called inside a for / if / while
impl<'a> Interpreter<'a> {
    fn run(&mut self) -> Result<(Variable, bool), String> {
        let mut statement_option = self.iter.next();
        while statement_option.is_some() {
            let statement = statement_option.ok_or("Some Looping went wrong!")?;
            match statement {
                // TODO exit is currently more of a return. just exits current context
                StatementNode::Exit(node) => {
                    self.out = (self.interpret_exit(node)?, true);
                    break;
                }
                StatementNode::Return(node) => {
                    self.out = (self.interpret_expr(node)?, true);
                    break;
                }
                StatementNode::Assign(name, node) => {
                    let value = self.interpret_expr(node)?;
                    self.vars.insert(name.to_owned(), value);
                }
                StatementNode::While(node) => {
                    self.interpret_while(node)?;
                    if self.out.1 {
                        break;
                    }
                }
                StatementNode::If(node) => {
                    self.interpret_if(node)?;
                    if self.out.1 {
                        break;
                    }
                }
                StatementNode::For(var, node) => {
                    self.interpret_for(var, node)?;
                    if self.out.1 {
                        break;
                    }
                }
                StatementNode::AssignIndex(name, inode, node) => {
                    self.interpret_assign_index(name, inode, node)?
                }
                StatementNode::Func(name, args) => self.interpret_func(name, args.to_owned())?,
                _ => return Err("Unknown statement".to_string()),
            }

            statement_option = self.iter.next();
        }
        return Ok(self.out.to_owned());
    }

    fn interpret_assign_index(
        &mut self,
        name: &str,
        inode: &ExpressionNode,
        node: &ExpressionNode,
    ) -> Result<(), String> {
        let value = self.interpret_expr(node)?;
        let index = self.interpret_expr(inode)?;
        let i = match index {
            Variable::Int(i) => i,
            _ => return Err("array index must be int".to_string()),
        };
        let array_var = self.vars.get_mut(name).ok_or("variable not found")?;
        let array = match array_var {
            Variable::Array(array) => array,
            _ => return Err("Can only index into arrays".to_string()),
        };
        array.borrow_mut()[i as usize] = value;
        Ok(())
    }

    fn interpret_func(&mut self, name: &str, args: Vec<String>) -> Result<(), String> {
        let mut nests = 1;
        let mut stmnts: Vec<StatementNode> = Vec::new();
        while nests != 0 {
            let next = self.iter.next().ok_or("Function not closed")?;
            match next {
                &StatementNode::EndFunc => nests -= 1,
                &StatementNode::Func(_, _) => nests += 1,
                _ => (),
            }
            stmnts.push(next.to_owned());
        }
        stmnts.pop();
        self.funcs.insert(name.to_string(), (args, stmnts));
        Ok(())
    }

    fn interpret_for(&mut self, var: &str, node: &ExpressionNode) -> Result<(), String> {
        let mut nests = 1;
        let mut stmnts: Vec<StatementNode> = Vec::new();
        while nests != 0 {
            let next = self.iter.next().ok_or("for loop not closed")?;
            match next {
                &StatementNode::EndFor => nests -= 1,
                &StatementNode::For(_, _) => nests += 1,
                _ => (),
            }
            stmnts.push(next.to_owned());
        }
        stmnts.pop();
        let array = match self.interpret_expr(node)? {
            Variable::Array(array) => array,
            _ => return Err("un iterable expression provided to for".to_string()),
        };
        for val in array.borrow_mut().iter_mut() {
            self.vars.insert(var.to_string(), val.to_owned());
            self.out = interpret_with_context(stmnts.iter(), self.vars, self.funcs)?;
        }
        Ok(())
    }

    // TODO Clean up repeated code. Functions and for loops will also likely be similar
    fn interpret_if(&mut self, node: &ExpressionNode) -> Result<(), String> {
        let mut nests = 1;
        let mut stmnts: Vec<StatementNode> = Vec::new();
        while nests != 0 {
            let next = self.iter.next().ok_or("if statement not closed")?;
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
            self.out = interpret_with_context(stmnts.iter(), self.vars, self.funcs)?;
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
            self.out = interpret_with_context(stmnts.iter(), self.vars, self.funcs)?;
        }
        Ok(())
    }

    fn interpret_exit(&mut self, node: &ExpressionNode) -> Result<Variable, String> {
        return match self.interpret_expr(node)? {
            Variable::Int(i) => Ok(Variable::Int(i)),
            _ => Err("wrong type passed to Exit".to_string()),
        };
    }

    fn interpret_expr(&mut self, node: &ExpressionNode) -> Result<Variable, String> {
        return match node {
            ExpressionNode::Int(value) => Ok(Variable::Int(
                value.parse::<i32>().ok().ok_or("invalid int".to_string())?,
            )),
            // TODO This will be awful for massive arrays
            // really we want to pass pointers to arrays and clone ints
            // for now we will just clone everything. Why would you need to references to an array?
            // can use Rc<RefCell> to achieve this if we want to implement in the future.
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
                _ => self.interpret_callables(name, nodes),
            },
            ExpressionNode::Index(name, node) => self.interpret_index(name, node),
            ExpressionNode::Array(nodes) => self.interpret_array(nodes),
            ExpressionNode::PreAllocArray(size) => {
                Ok(Variable::Array(RefCell::new(Vec::with_capacity(*size))))
            }
        };
    }

    // really we need to pass pointers to arrays here
    fn interpret_callables(
        &mut self,
        name: &str,
        nodes: &Vec<Box<ExpressionNode>>,
    ) -> Result<Variable, String> {
        let (args, code) = self
            .funcs
            .get(name)
            .ok_or("Function Not Defined")?
            .to_owned();
        if args.len() != nodes.len() {
            return Err("wrong number of args provided".to_string());
        }
        let mut context: HashMap<String, Variable> = HashMap::new();
        for (node, name) in nodes.iter().zip(args) {
            let var = self.interpret_expr(node)?;
            context.insert(name, var);
        }
        return Ok(interpret_with_context(code.iter(), &mut context, self.funcs)?.0);
    }

    fn interpret_index(&mut self, name: &str, node: &ExpressionNode) -> Result<Variable, String> {
        let i = match self.interpret_expr(node)? {
            Variable::Int(i) => i,
            _ => return Err("index must be int".to_string()),
        };
        let variable = self.vars.get(name).ok_or("Undefined var")?;
        return match variable {
            Variable::Array(array) => Ok(array.borrow()[i as usize].clone()),
            _ => Err("can only index into arrays".to_string()),
        };
    }

    fn interpret_array(&mut self, nodes: &Vec<Box<ExpressionNode>>) -> Result<Variable, String> {
        let mut array = Vec::with_capacity(nodes.len());
        for node in nodes.into_iter() {
            array.push(self.interpret_expr(node)?);
        }
        return Ok(Variable::Array(RefCell::new(array)));
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
