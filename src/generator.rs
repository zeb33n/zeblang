use crate::parser::{ExpressionNode, StatementNode};

use std::collections::HashMap;

#[derive(Debug)]
pub struct Generator {
    assembly: String,
    stack_pointer: i32,
    sp_cache: i32,
    loops: usize,
    ifs: usize,
    equalitys: usize,
    prints: usize,
    level: usize,
    context: String,
    funcs: HashMap<String, Vec<String>>,
    variables: HashMap<String, i32>,
}

impl Generator {
    pub fn new() -> Self {
        let asm = String::from(
            "section .data\n    msg: db 0, 0, 0, 0, 10\nsection .text\n    global _start\n_start:\n",
        );
        Self {
            assembly: asm,
            stack_pointer: 0,
            sp_cache: 0,
            loops: 0,
            ifs: 0,
            equalitys: 0,
            prints: 0,
            level: 1,
            context: "".to_string(),
            funcs: HashMap::new(),
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

    // should make this work for arbitary digits but this is fine for now
    // (even if it is 40 lines long lol)
    fn parse_print(&mut self) -> () {
        // load top of stack and calculate 1s, 10s, and, 100s
        self.generic("mov rax, [rsp]");
        self.generic("mov rbx, 100");
        self.generic("idiv rbx");
        self.generic("mov rcx, rax");
        self.generic("mov rax, rdx");
        self.generic("xor rdx, rdx");
        self.generic("mov rbx, 10 ");
        self.generic("idiv rbx");
        self.generic("mov rbx, rax");
        self.generic("mov eax, edx");

        // convert digits to ascii if above 0
        self.generic("add eax, '0'");
        self.generic("shl eax, 16");
        self.generic("mov ah, bl");
        self.generic("mov al, cl");
        self.generic("cmp al, 0");
        self.generic(&format!("je DIG2ASCII{}", self.prints));
        self.generic("add eax, '00'");
        self.generic(&format!("jmp ASCIIEX{}", self.prints));
        self.generic(&format!("DIG2ASCII{}:", self.prints));
        self.level += 1;
        self.generic("cmp ah, 0");
        self.generic(&format!("je ASCIIEX{}", self.prints));
        self.generic("add ah, '0'");
        self.level -= 1;
        self.generic(&format!("ASCIIEX{}:", self.prints));

        // use write syscall
        self.generic("mov [msg], eax");
        self.generic("mov rax, 1 ");
        self.generic("mov rdi, 1 ");
        self.generic("mov rsi, msg ");
        self.generic("mov rdx, 5");
        self.generic("syscall");
        self.generic("xor rax, rax");
        self.generic("xor rbx, rbx");
        self.generic("xor rcx, rcx");
        self.generic("xor rdx, rdx");
        self.prints += 1;
    }

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
            ExpressionNode::Callable(name, expr_vec) => {
                for expr in expr_vec.into_iter() {
                    self.generate_expr(*expr);
                }
                match name.as_str() {
                    "print" => self.parse_print(),
                    "range" => self.parse_range(),
                    name if self.funcs.contains_key(&name.to_string()) => {
                        self.generate_call_func(name.to_string())
                    }
                    _ => todo!("undeclared function"),
                }
            }
            ExpressionNode::Array(vector) => self.generate_array(vector),
            ExpressionNode::PreAllocArray(size) => self.generate_prealloc_array(size),
            ExpressionNode::Index(varname, expr) => self.generate_index(&varname, expr),
        }
    }

    fn generate_prealloc_array(&mut self, size: usize) -> () {
        for _ in 0..size + 1 {
            self.push("0x7F")
        }
    }

    fn generate_index(&mut self, varname: &str, expr: Box<ExpressionNode>) {
        self.generate_expr(*expr);
        self.pop("rbx");
        self.generic("mov rax, 8");
        self.generic("imul rbx");
        self.generic("mov rcx, rax");
        self.generic("mov rax, rsp");
        self.generic("sub rax, rcx");
        let key = format!("{}{}", self.context, varname);
        let variable_position = self.variables.get(&key).unwrap();
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
        self.generic("mov rax, 0x7F");
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

    fn get_var_pointer(&mut self, name: &str) -> String {
        let key = format!("{}{}", self.context, name);
        dbg!(&self.variables);
        dbg!(&key);
        let variable_position = self.variables.get(&key).unwrap();
        format!(
            "[rsp + {}]",
            (dbg!(self.stack_pointer) - variable_position - 1) * 8
        )
    }

    fn generate_assign(&mut self, mut name: String, node: ExpressionNode) -> () {
        name = format!("{}{}", self.context, &name);
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

    fn generate_assign_index(
        &mut self,
        mut name: String,
        index_expr: ExpressionNode,
        assign_expr: ExpressionNode,
    ) -> () {
        self.generate_expr(assign_expr);
        self.generate_expr(index_expr);
        self.pop("rcx");
        self.pop("rbx");
        self.generic("mov rax, 8");
        self.generic("imul rcx");
        self.generic("mov rcx, rax");
        self.generic("mov rax, rsp");
        self.generic("sub rax, rcx");
        name = format!("{}{}", self.context, &name);
        let variable_position = self.variables.get(&name).unwrap();
        let pointer = format!(
            "[rax + {}]",
            (self.stack_pointer - variable_position - 1) * 8
        );
        self.generic(&format!("mov {}, rbx", pointer))
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

    // should be able to raise an error
    // get rid of clone
    // arrays are broken. when reassigned only a referance to the first value is given.
    // need to add types decide how to implement array assigns -> pointer or copy -> maybe some
    // nice syntax.
    fn generate_for(&mut self, varname: String, node: ExpressionNode) -> () {
        // init var, pointer and loop
        self.generate_assign(format!("!LOOPARRAY{}", self.loops), node);
        self.generate_assign(varname.clone(), ExpressionNode::Value("0x7F".to_string()));
        self.generic("mov r8, 0");
        self.generic(&format!("FOR{}:", self.loops));
        self.level += 1;

        // increment array pointer and move value to var
        self.generic("mov rcx, r8");
        self.generic("mov rax, 8");
        self.generic("imul rcx");
        self.generic("mov rcx, rax");
        self.generic("mov rax, rsp");
        self.generic("sub rax, rcx");
        let variable_position = self
            .variables
            .get(&format!("!LOOPARRAY{}", self.loops))
            .unwrap();
        let pointer = format!(
            "[rax + {}]",
            (self.stack_pointer - variable_position - 1) * 8
        );
        let var = self.get_var_pointer(&varname);
        self.generic(&format!("mov rax, {}", pointer));
        self.generic(&format!("mov {}, rax", var));
        self.generic("inc r8");
        self.generic("xor rax, rax");
        self.generic("xor rcx, rcx");

        // are we at the end of the loop
        self.generic(&format!("mov rax, {}", var));
        self.generic("cmp rax, 0x7F");
        self.generic(&format!("je ENDFOR{}", self.loops));
        self.generic("xor rax, rax");
    }

    fn generate_end_for(&mut self) -> () {
        self.generic(&format!("jmp FOR{}", self.loops));
        self.level -= 1;
        self.generic(&format!("ENDFOR{}:", self.loops));
        self.loops += 1;
    }

    // how do i just take a reference of name and put it on the stack i guess is clone is just
    // doing the same thing but using the heap insted
    fn generate_func(&mut self, name: String, args: Vec<String>) -> () {
        for (i, arg) in args.iter().enumerate() {
            self.variables.insert(
                format!("{}{}", &name, arg),
                -(args.len() as i32 + 1) + i as i32,
            );
        }
        self.context = name.clone();
        self.sp_cache = self.stack_pointer;
        self.generic(&format!("jmp SKIP{}", &name));
        self.generic(&format!("{}:", &name));
        self.level += 1;
        self.push("rsp");
        self.push("rbp");
        self.generic("mov rbp, rsp");
        self.funcs.insert(name, args);
    }

    fn generate_end_func(&mut self) -> () {
        self.stack_pointer = self.sp_cache;
        self.generic("mov rsp, [rbp + 8]");
        self.generic("mov rbp, [rbp]");
        self.variables = self
            .variables
            .drain()
            .filter(|(key, _)| !key.contains(&self.context))
            .collect();
        self.generic(&format!("jmp END{}", self.context));
        self.level -= 1;
        self.generic(&format!("SKIP{}:", self.context));
        self.context = "".to_string();
    }

    fn generate_return(&mut self, node: ExpressionNode) -> () {
        self.generate_expr(node);
        self.pop("rax");
        self.generic("mov [rbp + 16], rax");
    }

    fn generate_call_func(&mut self, name: String) -> () {
        self.generic("mov rax, 0");
        self.push("rax");
        self.generic(&format!("jmp {}", name));
        self.generic(&format!("END{}:", &name));
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
                StatementNode::AssignIndex(name, index_expr, assign_expr) => {
                    self.generate_assign_index(name, index_expr, assign_expr)
                }
                StatementNode::EndFunc => self.generate_end_func(),
                StatementNode::Func(name, args) => self.generate_func(name, args),
                StatementNode::Return(expr) => self.generate_return(expr),
            };
        }
        self.assembly.to_owned()
    }
}
