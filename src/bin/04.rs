use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let lines = to_passport_lines(file_content);

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports_with_required_fields = lines
        .iter()
        .filter(|line| required_fields.iter().all(|x| line.contains(x)));

    println!(
        "Result of puzzle 1: {}",
        passports_with_required_fields.count()
    );
}

fn to_passport_lines(file_content: String) -> Vec<String> {
    let lines = file_content.lines();
    let mut passport_lines = Vec::new();
    let mut acc = "".to_string();

    for line in lines {
        if line == "" {
            passport_lines.push(acc);
            acc = "".to_string();
        } else if acc == "" {
            acc = line.to_string();
        } else {
            acc = format!("{} {}", acc, line.to_string());
        }
    }
    passport_lines.push(acc);
    passport_lines
}
