use crate::parser::{AssignNode, ExitNode, ExpressionNode, StatementNode};

use std::collections::HashMap;

#[derive(Debug)]
struct AssemblyData {
    assembly: String,
    stack_pointer: usize,
}

impl AssemblyData {
    fn new() -> Self {
        Self {
            assembly: String::from("global _start\n_start:\n"),
            stack_pointer: 0,
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
}

pub fn generate(program: Vec<StatementNode>) -> String {
    let mut assembly_data = AssemblyData::new();
    let mut variables: HashMap<String, usize> = HashMap::new();
    for line in program.into_iter() {
        match line {
            StatementNode::Exit(exit_node) => {
                let ExitNode::Expression(expr_node) = exit_node;
                let value = generate_expr(expr_node, &variables, &assembly_data.stack_pointer);
                assembly_data.generic(format!("mov rax, {}", value).as_str(), 1);
                assembly_data.push("rax", 1);
                assembly_data.generic("mov rax, 60", 1);
                assembly_data.pop("rdi", 1);
                assembly_data.generic("syscall", 1);
            }
            StatementNode::Assign(name, assign_node) => {
                variables.insert(name, assembly_data.stack_pointer);
                let AssignNode::Expression(expr_node) = assign_node;
                let value = generate_expr(expr_node, &variables, &assembly_data.stack_pointer);
                assembly_data.generic(format!("mov rax, {}", value).as_str(), 1);
                assembly_data.push("rax", 1);
            }
        };
    }
    assembly_data.assembly
}

fn generate_expr(
    expr: ExpressionNode,
    variables: &HashMap<String, usize>,
    stack_position: &usize,
) -> String {
    match expr {
        ExpressionNode::Value(value) => value,
        ExpressionNode::Var(value) => {
            let variable_position = variables.get(&value).unwrap();
            format!("[rsp + {}]", (stack_position - variable_position - 1) * 8)
        }
        ExpressionNode::Infix(_, _, _) => todo!("not implemented"),
    }
}
