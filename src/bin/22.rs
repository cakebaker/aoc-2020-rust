use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (cards_player_one, cards_player_two) = parse(file_content);
    let mut winning_score = play_combat(cards_player_one.clone(), cards_player_two.clone());

    println!("Result of puzzle 1: {}", winning_score);

    let (score_one, score_two) = play_recursive_combat(cards_player_one, cards_player_two);
    winning_score = if score_one == 0 { score_two } else { score_one };

    println!("Result of puzzle 2: {}", winning_score);
}

fn play_combat(a: VecDeque<usize>, b: VecDeque<usize>) -> usize {
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

fn play_recursive_combat(a: VecDeque<usize>, b: VecDeque<usize>) -> (usize, usize) {
    let mut cards_player_one = a;
    let mut cards_player_two = b;
    let mut history = Vec::new();

    loop {
        if history.contains(&cards_player_one) {
            // player one wins
            break (1, 0);
        }

        history.push(cards_player_one.clone());

        let card_player_one = cards_player_one.pop_front().unwrap();
        let card_player_two = cards_player_two.pop_front().unwrap();
        let mut player_one_wins_round = false;

        if cards_player_one.len() >= card_player_one && cards_player_two.len() >= card_player_two {
            let (score_one, _) = play_recursive_combat(
                cards_player_one
                    .iter()
                    .take(card_player_one)
                    .cloned()
                    .collect(),
                cards_player_two
                    .iter()
                    .take(card_player_two)
                    .cloned()
                    .collect(),
            );
            player_one_wins_round = score_one > 0;
        } else {
            player_one_wins_round = card_player_one > card_player_two;
        }

        if player_one_wins_round {
            cards_player_one.push_back(card_player_one);
            cards_player_one.push_back(card_player_two);
        } else {
            cards_player_two.push_back(card_player_two);
            cards_player_two.push_back(card_player_one);
        }

        if cards_player_one.is_empty() {
            break (0, calculate_score(cards_player_two));
        } else if cards_player_two.is_empty() {
            break (calculate_score(cards_player_one), 0);
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
