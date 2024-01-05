use crate::parser::{ExitNode, ExpressionNode, StatementNode};

pub fn generate(program: Vec<StatementNode>) -> String {
    dbg!(&program);
    let mut output = String::from("global _start\n_start:\n");
    for line in program.into_iter() {
        match line {
            StatementNode::Exit(exit_node) => {
                let ExitNode::Expression(expr_node) = exit_node;
                output += "   mov rax, 60\n";
                output += format!("   mov rdi, {}\n", generate_expression(expr_node)).as_str();
                output += "   syscall";
            }
            _ => panic!("syntax error"),
        };
    }
    output
}

fn generate_expression(expr: ExpressionNode) -> String {
    let ExpressionNode::Value(value) = expr else {
        todo!()
    };
    value
}
