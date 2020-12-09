use std::env;
use std::fs;

fn main() {
    const PRAEMBEL_LENGTH: usize = 25;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let numbers: Vec<usize> = file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let mut sums: Vec<usize> = Vec::new();
    for i in 0..PRAEMBEL_LENGTH {
        for j in 1..PRAEMBEL_LENGTH {
            sums.push(numbers[i] + numbers[i + j]);
        }
    }

    let mut result: usize = 0;
    for i in PRAEMBEL_LENGTH..numbers.len() {
        if sums.contains(&numbers[i]) {
            sums.drain(0..PRAEMBEL_LENGTH - 1);
            for j in 1..PRAEMBEL_LENGTH {
                sums.push(numbers[i] + numbers[i + j]);
            }
        } else {
            result = numbers[i];
            break;
        }
    }

    println!("Result of puzzle 1: {}", result);
}
