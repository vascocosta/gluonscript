use std::{collections::HashMap, io::stdin};

#[derive(Clone, Debug)]
pub enum Expr {
    Number(i64),
    String(String),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    ListLiteral(Vec<Expr>),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
}

impl Expr {
    fn eval_expr(expr: &Expr, env: &Env) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(*n),
            Expr::String(s) => Value::String(s.to_owned()),
            Expr::Variable(name) => env.get(name),
            Expr::Binary { left, op, right } => {
                let l = Self::eval_expr(left, env);
                let r = Self::eval_expr(right, env);

                match (l, r, op) {
                    (Value::Number(a), Value::Number(b), Operator::Add) => Value::Number(a + b),
                    (Value::Number(a), Value::Number(b), Operator::Sub) => Value::Number(a - b),
                    (Value::Number(a), Value::Number(b), Operator::Mul) => Value::Number(a * b),
                    (Value::Number(a), Value::Number(b), Operator::Greater) => Value::Bool(a > b),
                    (Value::Number(a), Value::Number(b), Operator::Smaller) => Value::Bool(a < b),
                    (Value::String(a), Value::String(b), Operator::Add) => {
                        Value::String(format!("{}{}", a, b))
                    }
                    _ => panic!("type error"),
                }
            }
            Expr::Call { name, args } => {
                let values: Vec<Value> = args.iter().map(|a| Self::eval_expr(a, env)).collect();

                if let Some(func) = env.functions.get(name) {
                    let mut local_env = env.child();

                    for (param, value) in func.params.iter().zip(values) {
                        local_env.set(param.clone(), value);
                    }

                    let mut result = Value::Bool(false);

                    for stmt in &func.body {
                        if let Some(v) = Program::exec_stmt(stmt, &mut local_env) {
                            result = v;
                        }
                    }

                    result
                } else {
                    call_builtin(name, &values)
                }
            }
            Expr::ListLiteral(elements) => {
                let values = elements.iter().map(|e| Self::eval_expr(e, env)).collect();

                Value::List(values)
            }
            Expr::Index { target, index } => {
                let list = Self::eval_expr(target, env);
                let idx = Self::eval_expr(index, env);

                match (list, idx) {
                    (Value::List(v), Value::Number(i)) => {
                        v.get(i as usize).cloned().expect("index out of bounds")
                    }

                    _ => panic!("invalid indexing"),
                }
            }
        }
    }
}

fn call_builtin(name: &str, args: &[Value]) -> Value {
    match name {
        "input" => {
            let mut buf: String = String::new();
            stdin().read_line(&mut buf).unwrap();

            Value::String(buf)
        }
        "print" | "println" => {
            for a in args {
                match a {
                    Value::Number(n) => print!("{}", n),
                    Value::Bool(b) => print!("{}", b),
                    Value::String(s) => print!("{}", s),
                    Value::List(l) => println!("{:#?}", l),
                }
            }

            if name == "println" {
                println!();
            }

            Value::Bool(true)
        }
        "number" => match &args[0] {
            Value::String(s) => Value::Number(
                s.trim_ascii()
                    .parse()
                    .expect("number expects a valid number string"),
            ),
            _ => panic!("number expects a string"),
        },
        "len" => match &args[0] {
            Value::List(v) => Value::Number(v.len() as i64),
            Value::String(s) => Value::Number(s.len() as i64),
            _ => panic!("len() unsupported type"),
        },
        "string" => match &args[0] {
            Value::Number(n) => Value::String(n.to_string()),
            _ => panic!("string expects a number"),
        },
        _ => panic!("unknown function {}", name),
    }
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Assign {
        name: String,
        value: Expr,
    },
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
    Sub,
    Greater,
    Smaller,
}

impl Operator {
    pub fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::Greater => 5,
            Operator::Smaller => 5,
            Operator::Add => 10,
            Operator::Sub => 10,
            Operator::Mul => 20,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
}

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Option<Value> {
        match stmt {
            Stmt::Assign { name, value } => {
                let v = Expr::eval_expr(value, env);
                env.vars.insert(name.clone(), v);
                None
            }
            Stmt::Expr(expr) => Some(Expr::eval_expr(expr, env)),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = Expr::eval_expr(condition, env);

                if let Value::Bool(true) = cond {
                    for stmt in then_branch {
                        Self::exec_stmt(stmt, env);
                    }
                } else {
                    for stmt in else_branch {
                        Self::exec_stmt(stmt, env);
                    }
                }

                None
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = Expr::eval_expr(condition, env);

                    match cond {
                        Value::Bool(true) => {
                            for stmt in body {
                                Self::exec_stmt(stmt, env);
                            }
                        }

                        Value::Bool(false) => break,

                        _ => panic!("while condition must be bool"),
                    }
                }

                None
            }
            Stmt::Function { name, params, body } => {
                env.functions.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                None
            }
        }
    }

    pub fn run(&self) -> Option<Value> {
        let mut env = Env::new();
        let mut last = None;

        for stmt in &self.statements {
            if let Some(v) = Self::exec_stmt(stmt, &mut env) {
                last = Some(v);
            }
        }

        last
    }
}

#[derive(Clone)]
struct Function {
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Clone)]
struct Env {
    vars: HashMap<String, Value>,
    functions: HashMap<String, Function>,
    parent: Option<Box<Env>>,
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    fn child(&self) -> Self {
        Self {
            vars: HashMap::new(),
            functions: self.functions.clone(),
            parent: Some(Box::new(self.clone())),
        }
    }

    fn get(&self, name: &str) -> Value {
        if let Some(v) = self.vars.get(name) {
            return v.clone();
        }

        if let Some(parent) = &self.parent {
            if let Some(v) = parent.vars.get(name) {
                return v.clone();
            }
        }

        panic!("undefined variable: {}", name)
    }

    fn set(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
}
