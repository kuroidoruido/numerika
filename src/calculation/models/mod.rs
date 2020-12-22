#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
    Factorial,
}

impl OperationUnit {
    #[inline]
    pub fn from_char(char_op: char) -> Option<Operator> {
        match char_op {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '!' => Some(Operator::Factorial),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorList {
    Unknown,
    CannotParseNumber(String),
    InvalidOperatorPosition,
    ThisOperatorShouldBeAlreadyComputed,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OperationUnit {
    Operand(f64),
    Operator(Operator),
}
