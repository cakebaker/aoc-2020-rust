use std::env;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (rules, _, other_tickets) = parse(file_content);
    let (_valid_tickets, invalid_tickets): (Vec<Ticket>, Vec<Ticket>) = other_tickets
        .into_iter()
        .partition(|ticket| is_valid_ticket(&rules, &ticket));
    let ticket_scanning_error_rate: usize = invalid_tickets
        .iter()
        .map(|ticket| get_invalid_value(&rules, ticket))
        .sum();

    println!("Result of puzzle 1: {}", ticket_scanning_error_rate);
}

fn is_valid_ticket(rules: &[Rule], ticket: &Ticket) -> bool {
    ticket.iter().all(|value| is_valid_value(&rules, *value))
}

fn is_valid_value(rules: &[Rule], value: usize) -> bool {
    for (_, ranges) in rules {
        if ranges[0].contains(&value) || ranges[1].contains(&value) {
            return true;
        }
    }
    false
}

fn get_invalid_value(rules: &[Rule], ticket: &Ticket) -> usize {
    for value in ticket {
        if !is_valid_value(rules, *value) {
            return *value;
        }
    }
    0
}

fn parse(file_content: String) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut rules = Vec::new();
    let mut other_tickets = Vec::new();

    let mut i = 0;
    while lines[i] != "" {
        let elems: Vec<&str> = lines[i].split(": ").collect();
        let field_name = elems[0].to_string();
        let rule: Vec<usize> = elems[1]
            .split(|c| c == '-' || c == ' ')
            .filter(|elem| *elem != "or")
            .map(|elem| elem.parse().unwrap())
            .collect();

        let mut ranges = Vec::new();
        for (i, _) in rule.iter().enumerate().step_by(2) {
            let range = rule[i]..=rule[i + 1];
            ranges.push(range);
        }

        rules.push((field_name, ranges));
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

type Rule = (String, Vec<RangeInclusive<usize>>);
type Ticket = Vec<usize>;
