use crate::error::new_error;
use crate::parser::{ExpressionNode, StatementNode};

use std::collections::HashMap;
use std::io::Result;

pub struct LlvmGenerator {
    ir: String,
    level: usize,
    ssa_counter: usize,
    variables: HashMap<String, usize>,
}

impl LlvmGenerator {
    pub fn new() -> Self {
        Self {
            ir: String::new(),
            level: 0,
            ssa_counter: 1,
            variables: HashMap::new(),
        }
    }

    pub fn generate(&mut self, program: Vec<StatementNode>) -> Result<String> {
        self.generic("declare void @exit(i32)");
        self.generic("define i32 @main() {");
        self.level += 1;
        for line in program.into_iter() {
            match line {
                StatementNode::Exit(expr_node) => self.generate_exit(expr_node)?,
                StatementNode::Assign(name, expr_node) => self.generate_assign(name, expr_node)?,
                StatementNode::For(var, expr_node) => todo!(),
                StatementNode::EndFor => todo!(),
                StatementNode::While(expr_node) => todo!(),
                StatementNode::EndWhile => todo!(),
                StatementNode::If(expr_node) => todo!(),
                StatementNode::EndIf => todo!(),
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
        self.generate_expr(node)?;
        self.generic(&format!("call void @exit(i32 %{})", self.ssa_counter - 1));
        Ok(())
    }

    fn generate_expr(&mut self, node: ExpressionNode) -> Result<()> {
        match node {
            ExpressionNode::Value(value) => {
                self.generic(&format!("%{} = add i32 0, {}", self.ssa_counter, value)); // FIX THIS HACK!
                self.ssa_counter += 1;
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn generate_assign(&mut self, name: String, node: ExpressionNode) -> Result<()> {
        self.generate_expr(node)?;
        if !self.variables.contains_key(&name) {
            self.generic(&format!("%{} = alloca i32, align 4", self.ssa_counter));
            self.variables.insert(name, self.ssa_counter);
            self.generic(&format!(
                "store i32 %{}, i32* %{}, align 4",
                self.ssa_counter - 1,
                self.ssa_counter
            ));
            self.ssa_counter += 1;
        } else {
            if let Some(register) = self.variables.get(&name) {
                self.generic(&format!(
                    "store i32 %{}, i32* %{}, align 4",
                    self.ssa_counter, register
                ));
            } else {
                return Err(new_error("variable not found"));
            }
        }
        Ok(())
    }
}
