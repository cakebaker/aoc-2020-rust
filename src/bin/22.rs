use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let (cards_player_one, cards_player_two) = parse(file_content);
    let winning_score_1 = play_combat(cards_player_one.clone(), cards_player_two.clone());

    println!("Result of puzzle 1: {}", winning_score_1);

    let (_, winning_score_2) = play_recursive_combat(cards_player_one, cards_player_two);

    println!("Result of puzzle 2: {}", winning_score_2);
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

fn play_recursive_combat(a: VecDeque<usize>, b: VecDeque<usize>) -> (Winner, usize) {
    let mut cards_player_one = a;
    let mut cards_player_two = b;
    let mut history = HashSet::new();

    loop {
        if history.contains(&cards_player_one) {
            break (Winner::Player1, 1);
        }

        history.insert(cards_player_one.clone());

        let card_player_one = cards_player_one.pop_front().unwrap();
        let card_player_two = cards_player_two.pop_front().unwrap();
        let round_winner: Winner;

        if cards_player_one.len() >= card_player_one && cards_player_two.len() >= card_player_two {
            let (sub_game_winner, _) = play_recursive_combat(
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
            round_winner = sub_game_winner;
        } else {
            round_winner = if card_player_one > card_player_two {
                Winner::Player1
            } else {
                Winner::Player2
            };
        }

        if round_winner == Winner::Player1 {
            cards_player_one.push_back(card_player_one);
            cards_player_one.push_back(card_player_two);
        } else {
            cards_player_two.push_back(card_player_two);
            cards_player_two.push_back(card_player_one);
        }

        if cards_player_one.is_empty() {
            break (Winner::Player2, calculate_score(cards_player_two));
        } else if cards_player_two.is_empty() {
            break (Winner::Player1, calculate_score(cards_player_one));
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
    let mut iter = file_content.split("\n\n");

    let cards_player_one = iter
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|elem| elem.parse().unwrap())
        .collect();
    let cards_player_two = iter
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|elem| elem.parse().unwrap())
        .collect();

    (cards_player_one, cards_player_two)
}

#[derive(PartialEq)]
enum Winner {
    Player1,
    Player2,
}
