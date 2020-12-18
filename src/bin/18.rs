use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let expressions = parse(file_content);
    let sum: usize = expressions
        .iter()
        .map(|expression| resolve(&expression))
        .sum();

    println!("Result of puzzle 1: {}", sum);
}

fn resolve(expression: &Expression) -> usize {
    let mut stack = Vec::new();

    for element in expression {
        match element {
            Element::Plus => {
                stack.push(Element::Plus);
            }
            Element::Times => {
                stack.push(Element::Times);
            }
            Element::OpenParen => {
                stack.push(Element::OpenParen);
            }
            Element::CloseParen => {
                if let Element::Int(a) = stack.pop().unwrap() {
                    stack.pop();
                    if stack.is_empty() || *stack.last().unwrap() == Element::OpenParen {
                        stack.push(Element::Int(a));
                    } else {
                        let operator = stack.pop().unwrap();
                        if let Element::Int(b) = stack.pop().unwrap() {
                            stack.push(perform_operation(operator, a, b));
                        }
                    }
                }
            }
            Element::Int(a) => {
                if stack.is_empty() || *stack.last().unwrap() == Element::OpenParen {
                    stack.push(Element::Int(*a));
                } else {
                    let operator = stack.pop().unwrap();
                    if let Element::Int(b) = stack.pop().unwrap() {
                        stack.push(perform_operation(operator, *a, b));
                    }
                }
            }
        }
    }

    if let Element::Int(a) = stack.pop().unwrap() {
        a
    } else {
        panic!("Unexpected element on stack!");
    }
}

fn perform_operation(operator: Element, a: usize, b: usize) -> Element {
    let result = if operator == Element::Plus {
        a + b
    } else {
        a * b
    };

    Element::Int(result)
}

fn parse(file_content: String) -> Vec<Expression> {
    let mut expressions = Vec::new();
    let lines: Vec<&str> = file_content.lines().collect();

    for line in lines {
        let mut expression = Vec::new();

        for c in line.chars() {
            match c {
                '+' => expression.push(Element::Plus),
                '*' => expression.push(Element::Times),
                '(' => expression.push(Element::OpenParen),
                ')' => expression.push(Element::CloseParen),
                '0'..='9' => expression.push(Element::Int(c.to_digit(10).unwrap() as usize)),
                _ => {}
            }
        }
        expressions.push(expression);
    }

    expressions
}

#[derive(Debug, PartialEq)]
enum Element {
    Plus,
    Times,
    OpenParen,
    CloseParen,
    Int(usize),
}

type Expression = Vec<Element>;
