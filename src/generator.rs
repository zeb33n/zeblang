use crate::parser::{ExpressionNode, StatementNode};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    assembly: String,
    stack_pointer: usize,
    loops: usize,
    ifs: usize,
    equalitys: usize,
    level: usize,
    variables: HashMap<String, usize>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            assembly: String::from("global _start\n_start:\n"),
            stack_pointer: 0,
            loops: 0,
            ifs: 0,
            equalitys: 0,
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

    fn get_var_pointer(&mut self, name: &str) -> String {
        let variable_position = self.variables.get(name).unwrap();
        format!(
            "[rsp + {}]",
            (self.stack_pointer - variable_position - 1) * 8
        )
    }

    fn generate_expr(&mut self, expr: ExpressionNode) -> () {
        match expr {
            ExpressionNode::Value(value) => {
                self.generic(format!("mov rax, {}", value).as_str());
                self.push("rax");
            }
            ExpressionNode::Var(name) => {
                let var = self.get_var_pointer(&name);
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
                    "/" => self.generic("idiv rbx"),
                    "%" => self.generate_modulo(),
                    "==" => self.generate_equality(),
                    "!=" => self.generate_inequality(),
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
            ExpressionNode::Array(vector) => self.generate_array(vector),
            ExpressionNode::Index(varname, expr) => self.generate_index(&varname, expr),
        }
    }

    // how to make arrays mutable -> needs more parsing!
    fn generate_index(&mut self, varname: &str, expr: Box<ExpressionNode>) {
        self.generate_expr(*expr);
        // needs to be [rsp + {} - rax * 8]
        // you cant sub a register in an effective address because it could pull you out of the
        // buffer. However, lets do something unsafe for now :')
        self.pop("rbx");
        self.generic("mov rax, 8");
        self.generic("imul rbx");
        self.generic("mov rcx, rax");
        self.generic("mov rax, rsp");
        self.generic("sub rax, rcx");
        let variable_position = self.variables.get(varname).unwrap();
        let index = format!(
            "[rax + {}]",
            (self.stack_pointer - variable_position - 1) * 8
        );
        self.generic(format!("mov rax, {}", index).as_str());
        self.push("rax");
    }

    fn generate_array(&mut self, vector: Vec<Box<ExpressionNode>>) -> () {
        for expr in vector.into_iter() {
            self.generate_expr(*expr);
        }
        self.generic("mov rax, 0x21"); // 0x21 is !
        self.push("rax");
    }

    fn generate_modulo(&mut self) -> () {
        self.generic("xor rdx, rdx"); // clear register xor is faster
        self.generic("idiv rbx");
        self.generic("mov rax, rdx");
    }

    fn generate_equality(&mut self) -> () {
        self.generic("cmp rax, rbx");
        self.generic(format!("je EQUALITY{}", self.equalitys).as_str());
        self.generic("mov rax, 0");
        self.generic(format!("jmp ENDEQ{}", self.equalitys).as_str());
        self.generic(format!("EQUALITY{}:", self.equalitys).as_str());
        self.level += 1;
        self.generic("mov rax, 1");
        self.level -= 1;
        self.generic(format!("ENDEQ{}:", self.equalitys).as_str());
        self.equalitys += 1;
    }

    fn generate_inequality(&mut self) -> () {
        self.generate_equality();
        self.generic("xor rax, 1");
    }

    fn generate_exit(&mut self, node: ExpressionNode) -> () {
        self.generate_expr(node);
        self.generic("mov rax, 60");
        self.pop("rdi");
        self.generic("syscall");
    }

    fn generate_assign(&mut self, name: String, node: ExpressionNode) -> () {
        if !self.variables.contains_key(&name) {
            self.variables.insert(name, self.stack_pointer);
            self.generate_expr(node);
        } else {
            self.generate_expr(node);
            self.pop("rax");
            let var = self.get_var_pointer(&name);
            self.generic(format!("mov {}, rax", var).as_str())
        };
    }

    fn generate_while(&mut self, node: ExpressionNode) -> () {
        self.generic(format!("wexp{}:", &self.loops).as_str());
        self.level += 1;
        self.generate_expr(node);
        self.pop("rax");
        self.generic("mov rbx, 0");
        self.generic("cmp rax, rbx");
        self.generic(format!("je exit{}", &self.loops).as_str());
        self.generic(format!("jmp loop{}", &self.loops).as_str());
        self.level -= 1;
        self.generic(format!("loop{}:", &self.loops).as_str());
        self.level += 1;
    }

    fn generate_end_while(&mut self) -> () {
        self.generic(format!("jmp wexp{}", &self.loops).as_str());
        self.level -= 1;
        self.generic(format!("exit{}:", &self.loops).as_str());
        self.loops += 1;
    }

    fn generate_if(&mut self, node: ExpressionNode) -> () {
        self.generate_expr(node);
        self.pop("rax");
        self.generic("cmp rax, 0");
        self.generic(format!("je endif{}", self.ifs).as_str());
    }

    fn generate_end_if(&mut self) -> () {
        self.generic(format!("endif{}:", self.ifs).as_str());
        self.ifs += 1;
    }

    fn generate_for(&mut self, var: String, node: ExpressionNode) -> () {
        (var, node); //silence warnings
        todo!()
    }

    fn generate_end_for(&mut self) -> () {
        todo!()
    }

    pub fn generate(&mut self, program: Vec<StatementNode>) -> String {
        dbg!(&program);
        for line in program.into_iter() {
            match line {
                StatementNode::Exit(expr_node) => self.generate_exit(expr_node),
                StatementNode::Assign(name, expr_node) => self.generate_assign(name, expr_node),
                StatementNode::For(var, expr_node) => self.generate_for(var, expr_node),
                StatementNode::EndFor => self.generate_end_for(),
                StatementNode::While(expr_node) => self.generate_while(expr_node),
                StatementNode::EndWhile => self.generate_end_while(),
                StatementNode::If(expr_node) => self.generate_if(expr_node),
                StatementNode::EndIf => self.generate_end_if(),
                StatementNode::AssignIndex(_, _, _) => todo!(),
            };
        }
        self.assembly.to_owned()
    }
}
