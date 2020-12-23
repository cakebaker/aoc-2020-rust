use std::collections::VecDeque;
use std::iter::FromIterator;

fn main() {
    const MAX_MOVES: usize = 100;
    let initial_cups: Vec<usize> = vec![3, 1, 8, 9, 4, 6, 5, 7, 2];

    let mut cups = CrabCups::new(initial_cups);

    for _ in 1..=MAX_MOVES {
        cups.do_move();
    }

    println!("Result of puzzle 1: {}", cups.get_labels_after_cup_one());
}

#[derive(Debug)]
struct CrabCups {
    cups: VecDeque<usize>,
    current_index: usize,
    length: usize,
}

impl CrabCups {
    fn new(xs: Vec<usize>) -> CrabCups {
        CrabCups {
            length: xs.len(),
            cups: VecDeque::from_iter(xs),
            current_index: 0,
        }
    }

    fn do_move(&mut self) {
        let mut i = self.current_index + 1;

        while i + 2 >= self.length {
            self.cups.rotate_left(1);
            self.current_index -= 1;
            i -= 1;
        }

        let cup_one = self.cups.remove(i).unwrap();
        let cup_two = self.cups.remove(i).unwrap();
        let cup_three = self.cups.remove(i).unwrap();

        let destination = self.find_destination();

        self.cups.insert((destination + 1) % self.length, cup_one);
        self.cups.insert((destination + 2) % self.length, cup_two);
        self.cups.insert((destination + 3) % self.length, cup_three);

        if destination < self.current_index {
            self.current_index = (self.current_index + 3) % self.length;
        }
        self.current_index = (self.current_index + 1) % self.length;
    }

    fn get_labels_after_cup_one(&self) -> String {
        let cups = Vec::from_iter(&self.cups);
        let mut iter = cups.split(|&cup| *cup == 1);
        let s1: String = iter
            .next()
            .unwrap()
            .iter()
            .map(|cup| cup.to_string())
            .collect();
        let s2: String = iter
            .next()
            .unwrap()
            .iter()
            .map(|cup| cup.to_string())
            .collect();

        format!("{}{}", s2, s1)
    }

    fn find_destination(&self) -> usize {
        let current = self.cups.get(self.current_index).unwrap();
        let mut needle = current - 1;

        while !self.cups.contains(&needle) {
            if needle == 0 {
                needle = 9;
            } else {
                needle -= 1;
            }
        }

        self.cups
            .iter()
            .enumerate()
            .filter(|(_, &cup)| cup == needle)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()[0]
    }
}
