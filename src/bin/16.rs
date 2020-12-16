use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (all_rules, _, other_tickets) = parse(file_content);
    let rules: Vec<usize> = all_rules.into_iter().flatten().collect();
    let tickets: Vec<usize> = other_tickets.into_iter().flatten().collect();

    let ticket_scanning_error_rate: usize = tickets
        .iter()
        .filter(|&ticket| !is_valid(&rules, *ticket))
        .sum();
    println!("Result of puzzle 1: {}", ticket_scanning_error_rate);
}

fn is_valid(rules: &[usize], ticket: usize) -> bool {
    for (i, _) in rules.iter().enumerate().step_by(2) {
        if ticket >= rules[i] && ticket <= rules[i + 1] {
            return true;
        }
    }
    false
}

fn parse(file_content: String) -> (Vec<Vec<usize>>, Vec<usize>, Vec<Vec<usize>>) {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut rules = Vec::new();
    let mut other_tickets = Vec::new();

    let mut i = 0;
    while lines[i] != "" {
        let elems: Vec<&str> = lines[i].split(": ").collect();
        let rule: Vec<usize> = elems[1]
            .split(|c| c == '-' || c == ' ')
            .filter(|elem| *elem != "or")
            .map(|elem| elem.parse().unwrap())
            .collect();
        rules.push(rule);
        i += 1;
    }

    i += 2;

    let my_ticket: Vec<usize> = lines[i]
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect();

    i += 3;

    while i < lines.len() {
        let other_ticket: Vec<usize> = lines[i]
            .split(',')
            .map(|elem| elem.parse().unwrap())
            .collect();
        other_tickets.push(other_ticket);
        i += 1;
    }

    (rules, my_ticket, other_tickets)
}
