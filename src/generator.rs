use crate::parser::{AssignNode, ExitNode, ExpressionNode, StatementNode};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    assembly: String,
    stack_pointer: usize,
    variables: HashMap<String, usize>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            assembly: String::from("global _start\n_start:\n"),
            stack_pointer: 0,
            variables: HashMap::new(),
        }
    }

    fn indent(level: usize) -> String {
        "    ".repeat(level).to_string()
    }

    fn push(&mut self, register: &str, level: usize) -> () {
        self.assembly += format!("{}push {}\n", Self::indent(level), register).as_str();
        self.stack_pointer += 1;
    }

    fn pop(&mut self, register: &str, level: usize) -> () {
        self.assembly += format!("{}pop {}\n", Self::indent(level), register).as_str();
        self.stack_pointer -= 1;
    }

    fn generic(&mut self, cmd: &str, level: usize) -> () {
        self.assembly += format!("{}{}\n", Self::indent(level), cmd).as_str();
    }

    fn generate_expr(&mut self, expr: ExpressionNode) -> () {
        match expr {
            ExpressionNode::Value(value) => {
                self.generic(format!("mov rax, {}", value).as_str(), 1);
                self.push("rax", 1);
            }
            ExpressionNode::Var(value) => {
                let variable_position = self.variables.get(&value).unwrap();
                let var = format!(
                    "[rsp + {}]",
                    (self.stack_pointer - variable_position - 1) * 8
                );
                self.generic(format!("mov rax, {}", var).as_str(), 1);
                self.push("rax", 1);
            }
            ExpressionNode::Infix(expr_1, _op, expr_2) => {
                self.generate_expr(*expr_1);
                self.generate_expr(*expr_2);
                self.pop("rax", 1);
                self.pop("rbx", 1);
                self.generic("add rax, rbx", 1);
                self.push("rax", 1);
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
                    self.generic("mov rax, 60", 1);
                    self.pop("rdi", 1);
                    self.generic("syscall", 1);
                }
                StatementNode::Assign(name, assign_node) => {
                    self.variables.insert(name, self.stack_pointer);
                    let AssignNode::Expression(expr_node) = assign_node;
                    self.generate_expr(expr_node);
                }
            };
        }
        self.assembly.to_owned()
    }
}
//
//global _start
//_start:
//    mov rax, 2
//    mov rbx, 3
//    add rax, rbx
//    push rax
//    mov rax, 60
//    pop rdi
//    syscall
