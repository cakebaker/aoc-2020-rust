use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let lines = to_passport_lines(file_content);

    let required_fields = get_required_fields();
    let passports_with_required_fields = lines
        .iter()
        .filter(|line| required_fields.iter().all(|field| line.contains(field)));

    println!(
        "Result of puzzle 1: {}",
        passports_with_required_fields.clone().count()
    );

    let valid_passports = passports_with_required_fields.filter(|line| is_valid(line));
    println!("Result of puzzle 2: {}", valid_passports.count());
}

fn is_valid(line: &str) -> bool {
    let validators = get_validators();
    let elements: Vec<&str> = line.split(|c| c == ' ' || c == ':').collect();
    elements
        .iter()
        .enumerate()
        .step_by(2)
        .all(|(i, field)| validators.get(*field).unwrap()(elements[i + 1]))
}

fn get_validators() -> HashMap<String, fn(&str) -> bool> {
    let mut validators: HashMap<String, fn(&str) -> bool> = HashMap::new();
    validators.insert("byr".to_string(), is_valid_birth_year);
    validators.insert("iyr".to_string(), is_valid_issue_year);
    validators.insert("eyr".to_string(), is_valid_expiration_year);
    validators.insert("hgt".to_string(), is_valid_height);
    validators.insert("hcl".to_string(), is_valid_hair_color);
    validators.insert("ecl".to_string(), is_valid_eye_color);
    validators.insert("pid".to_string(), is_valid_passport_id);
    validators.insert("cid".to_string(), is_valid_country_id);

    validators
}

fn get_required_fields() -> Vec<String> {
    let mut fields: Vec<String> = get_validators().keys().cloned().collect();
    fields.retain(|field| field != "cid");

    fields
}

fn is_valid_birth_year(birth_year: &str) -> bool {
    let year: usize = birth_year.parse().unwrap();

    year >= 1920 && year <= 2002
}

fn is_valid_issue_year(issue_year: &str) -> bool {
    let year: usize = issue_year.parse().unwrap();

    year >= 2010 && year <= 2020
}

fn is_valid_expiration_year(expiration_year: &str) -> bool {
    let year: usize = expiration_year.parse().unwrap();

    year >= 2020 && year <= 2030
}

fn is_valid_height(height_as_string: &str) -> bool {
    let length_without_unit = height_as_string.len() - 2;
    if height_as_string.contains("cm") {
        let height: usize = height_as_string[..length_without_unit].parse().unwrap();
        height >= 150 && height <= 193
    } else if height_as_string.contains("in") {
        let height: usize = height_as_string[..length_without_unit].parse().unwrap();
        height >= 59 && height <= 76
    } else {
        false
    }
}

fn is_valid_hair_color(hair_color: &str) -> bool {
    hair_color.len() == 7 && hair_color[1..].chars().all(|c| c.is_digit(16))
}

fn is_valid_eye_color(eye_color: &str) -> bool {
    let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    valid_eye_colors
        .iter()
        .any(|valid_eye_color| eye_color == *valid_eye_color)
}

fn is_valid_passport_id(passport_id: &str) -> bool {
    passport_id.len() == 9 && passport_id.parse::<f64>().is_ok()
}

fn is_valid_country_id(_: &str) -> bool {
    true
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
