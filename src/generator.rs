use crate::parser::Node;

pub fn generate(node: Node) -> String {
    let mut output = String::from("global _start\n_start:\n");
    match node {
        Node::Return(return_node) => {
            output += "   mov rax, 60\n";
            output += format!(
                "   mov rdi, {}\n",
                generate_expression(*return_node.expression)
            )
            .as_str();
            output += "   syscall";
        }
        _ => panic!("syntax error"),
    };
    output
}

fn generate_expression(node: Node) -> String {
    match node {
        Node::Expression(expression_node) => expression_node.integer,
        _ => panic!("syntax error"),
    }
}
