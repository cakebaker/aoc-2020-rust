use std::env;
use std::fs;

fn main() {
    const EXPECTED_EXPENSE_SUM: i32 = 2020;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let expenses: Vec<i32> = file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let (a, b) = find_expense_pair_with_sum(&expenses, EXPECTED_EXPENSE_SUM);
    println!("Result of puzzle 1: {}", a * b);
}

fn find_expense_pair_with_sum(expenses: &[i32], expected_sum: i32) -> (i32, i32) {
    let length = expenses.len();
    for i in 0..(length - 1) {
        for j in 1..(length) {
            if expenses[i] + expenses[j] == expected_sum {
                return (expenses[i], expenses[j]);
            }
        }
    }
    panic!("Error!");
}
