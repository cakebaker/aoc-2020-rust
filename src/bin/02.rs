use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let policies: Vec<PasswordPolicy> = file_content
        .lines()
        .map(|line| PasswordPolicy::new(line))
        .collect();

    println!(
        "Result of puzzle 1: {}",
        policies
            .iter()
            .filter(|p| is_valid_based_on_letter_count(*p))
            .count()
    );
    println!(
        "Result of puzzle 2: {}",
        policies
            .iter()
            .filter(|p| is_valid_based_on_letter_positions(*p))
            .count()
    );
}

fn is_valid_based_on_letter_count(policy: &PasswordPolicy) -> bool {
    let s: String = policy
        .password
        .chars()
        .filter(|c| *c == policy.letter)
        .collect();

    s.len() >= policy.low && s.len() <= policy.high
}

fn is_valid_based_on_letter_positions(policy: &PasswordPolicy) -> bool {
    (policy.password.chars().nth(policy.low - 1).unwrap() == policy.letter)
        ^ (policy.password.chars().nth(policy.high - 1).unwrap() == policy.letter)
}

#[derive(Debug)]
struct PasswordPolicy {
    low: usize,
    high: usize,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn new(line: &str) -> PasswordPolicy {
        let v: Vec<&str> = line.split(|c| c == ' ' || c == '-').collect();
        PasswordPolicy {
            low: v[0].parse().unwrap(),
            high: v[1].parse().unwrap(),
            letter: v[2].chars().next().unwrap(),
            password: v[3].to_string(),
        }
    }
}
