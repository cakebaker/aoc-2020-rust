use std::env;
use std::fs;

fn main() {
    const SUBJECT_NUMBER: usize = 7;
    const DIVISOR: usize = 20201227;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (card_public_key, door_public_key) = parse(file_content);
    let loop_size = find_secret_loop_size(card_public_key, SUBJECT_NUMBER, DIVISOR);
    let encryption_key = find_encryption_key(loop_size, door_public_key, DIVISOR);

    println!("Result of puzzle 1: {}", encryption_key);
}

fn find_secret_loop_size(key: usize, subject_number: usize, divisor: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;

    while value != key {
        value *= subject_number;
        value %= divisor;
        loop_size += 1;
    }

    loop_size
}

fn find_encryption_key(loop_size: usize, subject_number: usize, divisor: usize) -> usize {
    let mut value = 1;

    for _ in 1..=loop_size {
        value *= subject_number;
        value %= divisor;
    }

    value
}

fn parse(file_content: String) -> (usize, usize) {
    let public_keys: Vec<usize> = file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (public_keys[0], public_keys[1])
}
