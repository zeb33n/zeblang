use crate::error::new_error;
use crate::parser::{ExpressionNode, StatementNode};

use std::collections::HashMap;
use std::io::Result;

pub struct LlvmGenerator {
    ir: String,
    level: usize,
    ssa_counter: usize,
    variables: HashMap<String, usize>,
    ifs: usize,
}

impl LlvmGenerator {
    pub fn new() -> Self {
        Self {
            ir: String::new(),
            level: 0,
            ssa_counter: 1,
            variables: HashMap::new(),
            ifs: 0,
        }
    }

    pub fn generate(&mut self, program: Vec<StatementNode>) -> Result<String> {
        self.generic("declare void @exit(i32)");
        self.generic("define i32 @main() {");
        self.generic("entry:");
        self.level += 1;
        for line in program.into_iter() {
            match line {
                StatementNode::Exit(expr_node) => self.generate_exit(expr_node)?,
                StatementNode::Assign(name, expr_node) => self.generate_assign(name, expr_node)?,
                StatementNode::For(var, expr_node) => todo!(),
                StatementNode::EndFor => todo!(),
                StatementNode::While(expr_node) => todo!(),
                StatementNode::EndWhile => todo!(),
                StatementNode::If(expr_node) => self.generate_if(expr_node)?,
                StatementNode::EndIf => self.generate_end_if(),
                StatementNode::AssignIndex(name, index_expr, assign_expr) => todo!(),
                StatementNode::EndFunc => todo!(),
                StatementNode::Func(name, args) => todo!(),
                StatementNode::Return(expr) => todo!(),
            };
        }
        self.generic("ret i32 1");
        self.level -= 1;
        self.generic("}");
        Ok(self.ir.to_owned())
    }

    fn generic(&mut self, cmd: &str) -> () {
        self.ir += format!("{}{}\n", Self::indent(self.level), cmd).as_str();
    }

    fn indent(level: usize) -> String {
        "    ".repeat(level).to_string()
    }

    fn generate_exit(&mut self, node: ExpressionNode) -> Result<()> {
        let value = self.generate_expr(node)?;
        self.generic(&format!("call void @exit(i32 {})", value));
        Ok(())
    }

    fn generate_expr(&mut self, node: ExpressionNode) -> Result<String> {
        match node {
            ExpressionNode::Value(value) => Ok(value),
            ExpressionNode::Var(name) => {
                let e = new_error(&format!("Variable {} not found", &name));
                let ssa_reg = self.variables.get(&name).ok_or(e)?;
                let load_reg = format!("%{}", self.ssa_counter);
                self.ssa_counter += 1;
                self.generic(&format!(
                    "{} = load i32, i32* %{}, align 4",
                    load_reg, ssa_reg
                ));
                Ok(load_reg)
            }
            ExpressionNode::Infix(expr1, infix, expr2) => {
                let left = self.generate_expr(*expr1)?;
                let right = self.generate_expr(*expr2)?;
                let load_reg = format!("%{}", self.ssa_counter);
                self.ssa_counter += 1;
                match infix.as_str() {
                    "+" => self.generic(&format!("{load_reg} = add i32 {left}, {right}")),
                    "-" => self.generic(&format!("{load_reg} = sub i32 {left}, {right}")),
                    "/" => self.generic(&format!("{load_reg} = udiv i32 {left}, {right}")),
                    "*" => self.generic(&format!("{load_reg} = mul i32 {left}, {right}")),
                    "%" => self.generic(&format!("{load_reg} = urem i32 {left}, {right}")),
                    "==" => self.generic(&format!("{load_reg} = icmp eq i32 {left}, {right}")),
                    "!=" => self.generic(&format!("{load_reg} = icmp ne i32 {left}, {right}")),
                    ">" => self.generic(&format!("{load_reg} = icmp ugt {left}, {right}")),
                    ">=" => self.generic(&format!("{load_reg} = icmp uge {left}, {right}")),
                    "<" => self.generic(&format!("{load_reg} = icmp ult {left}, {right}")),
                    "<=" => self.generic(&format!("{load_reg} = icmp ule {left}, {right}")),
                    _ => todo!(),
                };
                Ok(load_reg)
            }
            // need to use ptr type / implement types
            ExpressionNode::Array(exprs) => {
                let array_reg = format!("%{}", self.ssa_counter);
                let array_len = &exprs.len();
                let cmd = &format!("{array_reg} = alloca [{array_len} x i32], align 4");
                self.ssa_counter += 1;
                self.generic(cmd);
                let mut counter = 0;
                for expr in exprs.into_iter() {
                    let value = self.generate_expr(*expr)?;
                    self.generic(&format!(
                        "%{} = getelementptr inbounds [{} x i32], [{} x i32]* {}, i32 0, i32 {}",
                        self.ssa_counter, array_len, array_len, array_reg, counter
                    ));
                    self.generic(&format!(
                        "store i32 {}, i32* %{}, align 4",
                        value, self.ssa_counter
                    ));
                    counter += 1;
                    self.ssa_counter += 1;
                }
                Ok(array_reg)
            }
            // the string name should really just be am expr here.
            // need to implement ptr types
            ExpressionNode::Index(name, expr) => {
                let value = self.generate_expr(*expr)?;
                let e = new_error(&format!("Variable {} not found", &name));
                let ssa_reg = self.variables.get(&name).ok_or(e)?;
                let load_reg = format!("%{}", self.ssa_counter);
                self.ssa_counter += 1;
                let ptr_reg = format!("%{}", self.ssa_counter);
                self.ssa_counter += 1;
                self.generic(&format!(
                    "{ptr_reg} = getelementptr i32* {ssa_reg}, i32 0, i32 {value}",
                ));
                self.generic(&format!("{load_reg} = load i32, i32* {ptr_reg}, align 4"));

                Ok(load_reg)
            }

            _ => todo!(),
        }
    }

    fn generate_assign(&mut self, name: String, node: ExpressionNode) -> Result<()> {
        let value = self.generate_expr(node)?;
        if !self.variables.contains_key(&name) {
            self.generic(&format!("%{} = alloca i32, align 4", self.ssa_counter));
            self.variables.insert(name, self.ssa_counter);
            self.generic(&format!(
                "store i32 {}, i32* %{}, align 4",
                value, self.ssa_counter
            ));
            self.ssa_counter += 1;
        } else {
            if let Some(register) = self.variables.get(&name) {
                self.generic(&format!("store i32 {}, i32* %{}, align 4", value, register));
            } else {
                return Err(new_error("variable not found"));
            }
        }
        Ok(())
    }

    fn generate_if(&mut self, node: ExpressionNode) -> Result<()> {
        let value = self.generate_expr(node)?;
        self.generic(&format!(
            "br i1 {}, label %true{}, label %end{}",
            value, self.ifs, self.ifs
        ));
        self.level -= 1;
        self.generic(&format!("true{}:", self.ifs));
        self.level += 1;
        Ok(())
    }

    // TODO look into phi opcode for defining variables inside if statements
    fn generate_end_if(&mut self) -> () {
        self.generic(&format!("br label %end{}", self.ifs));
        self.level -= 1;
        self.generic(&format!("end{}:", self.ifs));
        self.ifs += 1;
        self.level += 1;
    }
}
