pub mod lexer;
mod expression;

use lexer::{Lexer, Token};
use expression::Expression;

use std::io::stdin;

const FORMAT_ERROR: &str = "Input has an invalid format.";

fn parse(input: Vec<char>) -> Result<Expression, String> {
    let mut lexer = Lexer::new(input);
    let mut stack = Vec::<Expression>::new();
    while let Some(token) = lexer.read_token() {
        let expression = match token {
            Token::Unknown(character) => Err(format!("Unexpected character '{}' found.", character))?,
            Token::Space => continue,
            Token::Number(number) => Expression::Literal(number),
            Token::Plus => {
                let right = stack.pop().ok_or(FORMAT_ERROR)?;
                let left = stack.pop().ok_or(FORMAT_ERROR)?;
                Expression::Addition(Box::new(left), Box::new(right))
            }
            Token::Minus => {
                let right = stack.pop().ok_or(FORMAT_ERROR)?;
                let left = stack.pop().ok_or(FORMAT_ERROR)?;
                Expression::Subtraction(Box::new(left), Box::new(right))
            }
            Token::Asterisk => {
                let right = stack.pop().ok_or(FORMAT_ERROR)?;
                let left = stack.pop().ok_or(FORMAT_ERROR)?;
                Expression::Multiplication(Box::new(left), Box::new(right))
            }
            Token::Slash => {
                let right = stack.pop().ok_or(FORMAT_ERROR)?;
                let left = stack.pop().ok_or(FORMAT_ERROR)?;
                Expression::Division(Box::new(left), Box::new(right))
            }
        };
        stack.push(expression);
    }
    match stack.len() {
        1 => Ok(stack.pop().unwrap()),
        _ => Err(FORMAT_ERROR.to_string())
    }
}

fn main() {
    let mut input = String::new();

    if let Err(error) = stdin().read_line(&mut input) {
        println!("{error}");
    }

    let characters = input
        .trim()
        .chars()
        .collect();
    
    let result = parse(characters)
        .and_then(|expression| {
            let result = expression.evaluate();
            Ok((expression, result))
        });
    
    match result {
        Err(error) => println!("{error}"),
        Ok((expression, Err(error))) => println!("{expression} resulted in error: {error}"),
        Ok((expression, Ok(result))) => println!("{expression} = {result}"),
    }
}
