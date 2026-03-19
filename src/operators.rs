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
    EqualEqual,
    Or,
    And,
}

impl Operator {
    pub fn precedence(op: &Operator) -> u8 {
        match op {
            Operator::Or => 1,
            Operator::And => 2,

            Operator::EqualEqual => 3,

            Operator::Greater => 4,
            Operator::GreaterEqual => 4,
            Operator::Smaller => 4,
            Operator::SmallerEqual => 4,

            Operator::Add => 5,
            Operator::Sub => 5,

            Operator::Mul => 6,
            Operator::Div => 6,
            Operator::Percent => 6,
        }
    }
}
