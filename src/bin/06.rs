use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");
    let lines: Vec<&str> = file_content.lines().collect();

    let mut group: Vec<&str> = Vec::new();
    let mut count = 0;

    for line in lines {
        if line == "" {
            count += count_answered_questions(group.join(""));
            group.clear();
        } else {
            group.push(line);
        }
    }
    count += count_answered_questions(group.join(""));

    println!("Result of puzzle 1: {}", count);
}

fn count_answered_questions(answers: String) -> usize {
    let mut chars: Vec<char> = answers.chars().collect();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}
