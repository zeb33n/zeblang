use crate::parser::{AssignNode, ExitNode, ExpressionNode, StatementNode};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    assembly: String,
    stack_pointer: usize,
    loops: usize,
    level: usize,
    variables: HashMap<String, usize>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            assembly: String::from("global _start\n_start:\n"),
            stack_pointer: 0,
            loops: 0,
            level: 1,
            variables: HashMap::new(),
        }
    }

    fn indent(level: usize) -> String {
        "    ".repeat(level).to_string()
    }

    fn push(&mut self, register: &str) -> () {
        self.assembly += format!("{}push {}\n", Self::indent(self.level), register).as_str();
        self.stack_pointer += 1;
    }

    fn pop(&mut self, register: &str) -> () {
        self.assembly += format!("{}pop {}\n", Self::indent(self.level), register).as_str();
        self.stack_pointer -= 1;
    }

    fn generic(&mut self, cmd: &str) -> () {
        self.assembly += format!("{}{}\n", Self::indent(self.level), cmd).as_str();
    }

    // more research needed
    fn parse_print(&mut self) -> () {
        todo!();
    }

    // add a speacial terminator value like 0x?? or something
    fn parse_range(&mut self) -> () {
        self.pop("rax");
        self.generic("mov rbx, 0");
        self.generic(format!("loop{}:", &self.loops).as_str());
        self.level += 1;
        self.push("rbx");
        self.generic("inc rbx");
        self.generic("cmp rax, rbx");
        self.generic(format!("je exit{}", &self.loops).as_str());
        self.generic(format!("jmp loop{}", &self.loops).as_str());
        self.level -= 1;
        self.generic(format!("exit{}:", &self.loops).as_str());
        self.loops += 1;
    }

    fn generate_expr(&mut self, expr: ExpressionNode) -> () {
        match expr {
            ExpressionNode::Value(value) => {
                self.generic(format!("mov rax, {}", value).as_str());
                self.push("rax");
            }
            ExpressionNode::Var(value) => {
                let variable_position = self.variables.get(&value).unwrap();
                let position = if variable_position == &self.stack_pointer {
                    *variable_position
                } else {
                    self.stack_pointer - variable_position
                };
                let var = format!("[rsp + {}]", (position - 1) * 8);
                self.generic(format!("mov rax, {}", var).as_str());
                self.push("rax");
            }
            ExpressionNode::Infix(expr_1, op, expr_2) => {
                self.generate_expr(*expr_1);
                self.generate_expr(*expr_2);
                self.pop("rbx");
                self.pop("rax");
                match op.as_str() {
                    "+" => self.generic("add rax, rbx"),
                    "-" => self.generic("sub rax, rbx"),
                    "*" => self.generic("imul rbx"),
                    _ => todo!(),
                }
                self.push("rax");
            }
            ExpressionNode::Callable(name, expr) => {
                self.generate_expr(*expr);
                match name.as_str() {
                    "print(" => self.parse_print(),
                    "range(" => self.parse_range(),
                    _ => todo!("undeclared function"),
                }
            }
        }
    }

    pub fn generate(&mut self, program: Vec<StatementNode>) -> String {
        dbg!(&program);
        for line in program.into_iter() {
            match line {
                StatementNode::Exit(exit_node) => {
                    let ExitNode::Expression(expr_node) = exit_node;
                    self.generate_expr(expr_node);
                    self.generic("mov rax, 60");
                    self.pop("rdi");
                    self.generic("syscall");
                }
                StatementNode::Assign(name, assign_node) => {
                    self.variables.insert(name, self.stack_pointer);
                    let AssignNode::Expression(expr_node) = assign_node;
                    self.generate_expr(expr_node);
                }
                StatementNode::For(var, expr_node) => {
                    self.generic(format!("jmp loop{}", &self.loops).as_str());
                    self.variables.insert(var, self.stack_pointer);
                    self.generate_expr(expr_node);
                    todo!()
                }
                StatementNode::EndFor => {
                    self.generic(format!("jmp loop{}", &self.loops).as_str());
                    todo!()
                }
                StatementNode::While(expr_node) => {
                    // the conditional part of the while
                    self.generic(format!("wexp{}:", &self.loops).as_str());
                    self.level += 1;
                    self.generate_expr(expr_node);
                    self.pop("rax");
                    self.generic("mov rbx, 0");
                    self.generic("cmp rax, rbx");
                    self.generic(format!("je exit{}", &self.loops).as_str());
                    self.generic(format!("jmp loop{}", &self.loops).as_str());
                    self.level -= 1;

                    //enter the loop
                    //variables arent in the smae place on the stack after every loop !?
                    self.generic(format!("loop{}:", &self.loops).as_str());
                    self.level += 1;
                }
                //wont work with nested loops?!
                StatementNode::EndWhile => {
                    self.generic(format!("jmp wexp{}", &self.loops).as_str());
                    self.level -= 1;
                    self.generic(format!("exit{}:", &self.loops).as_str());
                    self.loops += 1;
                }
            };
        }
        self.assembly.to_owned()
    }
}

// we can generate nodes based on this structure
// start for node
// statement node
// statement node
// ...
// end for node
