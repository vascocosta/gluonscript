#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Mul,
    Sub,
    Div,
    Percent,
    Greater,
    GreaterEqual,
    Smaller,
    SmallerEqual,
    Equal,
    Or,
    And,
}

impl Operator {
    pub fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::Greater => 5,
            Operator::GreaterEqual => 5,
            Operator::Smaller => 5,
            Operator::SmallerEqual => 5,
            Operator::Add => 10,
            Operator::Sub => 10,
            Operator::Mul => 20,
            Operator::Div => 20,
            Operator::Percent => 10,
            Operator::Equal => 3,
            Operator::Or => 2,
            Operator::And => 2,
        }
    }
}
