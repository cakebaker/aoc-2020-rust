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
    let valid_password_count = policies.iter().filter(|p| p.is_valid()).count();
    let valid_password_count2 = policies.iter().filter(|p| p.is_valid2()).count();
    println!("Result of puzzle 1: {}", valid_password_count);
    println!("Result of puzzle 2: {}", valid_password_count2);
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

    fn is_valid(&self) -> bool {
        let s: String = self
            .password
            .chars()
            .filter(|c| *c == self.letter)
            .collect();

        s.len() >= self.low && s.len() <= self.high
    }

    fn is_valid2(&self) -> bool {
        (self.password.chars().nth(self.low - 1).unwrap() == self.letter)
            ^ (self.password.chars().nth(self.high - 1).unwrap() == self.letter)
    }
}
