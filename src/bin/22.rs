use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (cards_player_one, cards_player_two) = parse(file_content);
    let score = play(cards_player_one, cards_player_two);

    println!("Result of puzzle 1: {}", score);
}

fn play(a: VecDeque<usize>, b: VecDeque<usize>) -> usize {
    let mut cards_player_one = a;
    let mut cards_player_two = b;

    loop {
        let card_player_one = cards_player_one.pop_front().unwrap();
        let card_player_two = cards_player_two.pop_front().unwrap();

        if card_player_one > card_player_two {
            cards_player_one.push_back(card_player_one);
            cards_player_one.push_back(card_player_two);
        } else {
            cards_player_two.push_back(card_player_two);
            cards_player_two.push_back(card_player_one);
        }

        if cards_player_one.is_empty() {
            break calculate_score(cards_player_two);
        } else if cards_player_two.is_empty() {
            break calculate_score(cards_player_one);
        }
    }
}

fn calculate_score(cards: VecDeque<usize>) -> usize {
    cards
        .iter()
        .enumerate()
        .map(|(i, elem)| elem * (cards.len() - i))
        .sum()
}

fn parse(file_content: String) -> (VecDeque<usize>, VecDeque<usize>) {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut cards_player_one = VecDeque::new();
    let mut cards_player_two = VecDeque::new();

    let mut i = 1;

    while lines[i] != "" {
        cards_player_one.push_back(lines[i].parse().unwrap());
        i += 1;
    }

    i += 2;

    while i < lines.len() {
        cards_player_two.push_back(lines[i].parse().unwrap());
        i += 1;
    }

    (cards_player_one, cards_player_two)
}
