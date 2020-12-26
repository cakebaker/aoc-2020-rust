use std::env;
use std::fs;

fn main() {
    const PREAMBLE_LENGTH: usize = 25;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let numbers: Vec<usize> = file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let invalid_number = find_invalid_number(&numbers, PREAMBLE_LENGTH);

    println!("Result of puzzle 1: {}", invalid_number);
    println!(
        "Result of puzzle 2: {}",
        find_encryption_weakness(&numbers, invalid_number)
    );
}

fn find_invalid_number(numbers: &[usize], preamble_length: usize) -> usize {
    let mut sums: Vec<usize> = Vec::new();
    let mut result: usize = 0;

    for i in 0..preamble_length {
        for j in 1..preamble_length {
            sums.push(numbers[i] + numbers[i + j]);
        }
    }

    for i in preamble_length..numbers.len() {
        if sums.contains(&numbers[i]) {
            sums.drain(0..preamble_length - 1);
            for j in 1..preamble_length {
                sums.push(numbers[i] + numbers[i + j]);
            }
        } else {
            result = numbers[i];
            break;
        }
    }
    result
}

fn find_encryption_weakness(numbers: &[usize], invalid_number: usize) -> usize {
    let mut sum: usize = 0;
    let mut sum_elements: Vec<usize> = Vec::new();
    let mut result: usize = 0;

    for i in numbers {
        if sum + i < invalid_number {
            sum += i;
            sum_elements.push(*i);
        } else {
            if sum + i > invalid_number {
                while sum + i > invalid_number {
                    sum -= sum_elements.remove(0);
                }
                sum_elements.push(*i);
                sum += i;
            }

            if sum == invalid_number {
                sum_elements.sort_unstable();
                result = sum_elements.first().unwrap() + sum_elements.last().unwrap();
                break;
            }
        }
    }
    result
}
