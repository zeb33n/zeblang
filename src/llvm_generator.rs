use crate::error::new_error;
use crate::parser::{ExpressionNode, StatementNode};

use std::collections::HashMap;
use std::io::Result;

pub struct LlvmGenerator {
    ir: String,
    level: usize,
    sass_counter: usize,
    variables: HashMap<String, usize>,
}

impl LlvmGenerator {
    pub fn new() -> Self {
        Self {
            ir: String::new(),
            level: 0,
            sass_counter: 1,
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
        self.generic(&format!(
            "%{} = load i32, i32* %{}, align 4",
            self.sass_counter,
            self.sass_counter - 1
        ));
        self.generic(&format!("call void @exit(i32 %{})", self.sass_counter));
        Ok(())
    }

    fn generate_expr(&mut self, node: ExpressionNode) -> Result<()> {
        match node {
            ExpressionNode::Value(value) => {
                self.generic(&format!("%{} = alloca i32, align 4", self.sass_counter));
                self.generic(&format!(
                    "store i32 {}, i32* %{}, align 4",
                    value, self.sass_counter
                ));
                self.sass_counter += 1;
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn generate_assign(&mut self, name: String, node: ExpressionNode) -> Result<()> {
        self.generate_expr(node)?;
        self.variables.insert(name, self.sass_counter);
        self.generic(&format!(
            "%{} = load i32, i32* %{}, align 4",
            self.sass_counter,
            self.sass_counter - 1
        ));
        self.sass_counter += 1;
        self.generic(&format!("%{} = alloca i32, align 4", self.sass_counter));
        self.generic(&format!(
            "store i32 %{}, i32* {}, align 4",
            self.sass_counter - 1,
            self.sass_counter
        ));
        Ok(())
    }
}
