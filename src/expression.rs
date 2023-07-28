use std::fmt::Display;

#[derive(Debug)]
pub enum Expression {
    Literal(i32),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
}

const ZERO_DIVISION_ERROR: &str = "Division by zero.";

impl Expression {
    pub fn evaluate(&self) -> Result<i32, String> {
        // println!("Evaluating expression {:?}", *self);
        match self {
            Expression::Literal(number) => Ok(*number),
            Expression::Addition(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                Ok(left + right)
            },
            Expression::Subtraction(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                Ok(left - right)
            },
            Expression::Multiplication(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                Ok(left * right)
            },
            Expression::Division(left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                if right == 0 {
                    return Err(ZERO_DIVISION_ERROR.to_string());
                }
                Ok(left / right)
            },
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(number) => write!(f, "{}", number),
            Expression::Addition(left, right) => write!(f, "({} + {})", *left, *right),
            Expression::Subtraction(left, right) => write!(f, "({} - {})", *left, *right),
            Expression::Multiplication(left, right) => write!(f, "({} * {})", *left, *right),
            Expression::Division(left, right) => write!(f, "({} / {})", *left, *right),
        }
    }
}