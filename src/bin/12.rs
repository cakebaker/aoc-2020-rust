use std::env;
use std::fs;

fn main() {
    const INITIAL_DIRECTION: isize = 90;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let actions: Vec<Action> = file_content.lines().map(|line| parse(line)).collect();

    let mut direction = FacingDirection::new(INITIAL_DIRECTION);
    let mut east_west: isize = 0;
    let mut north_south: isize = 0;

    for action in &actions {
        match action {
            Action::North(v) => north_south += v,
            Action::South(v) => north_south -= v,
            Action::East(v) => east_west += v,
            Action::West(v) => east_west -= v,
            Action::Left(v) => direction.turn_left(*v),
            Action::Right(v) => direction.turn_right(*v),
            Action::Forward(v) => match direction.current() {
                Direction::North => north_south += v,
                Direction::South => north_south -= v,
                Direction::East => east_west += v,
                Direction::West => east_west -= v,
            },
        };
    }

    println!(
        "Result of puzzle 1: {}",
        east_west.abs() + north_south.abs()
    );
}

fn parse(s: &str) -> Action {
    let (first, last) = s.split_at(1);
    let value = last.parse().unwrap();

    match first {
        "N" => Action::North(value),
        "S" => Action::South(value),
        "E" => Action::East(value),
        "W" => Action::West(value),
        "L" => Action::Left(value),
        "R" => Action::Right(value),
        _ => Action::Forward(value),
    }
}

#[derive(Debug)]
enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct FacingDirection {
    degrees: isize,
}

impl FacingDirection {
    fn new(i: isize) -> FacingDirection {
        FacingDirection { degrees: i }
    }

    fn turn_left(&mut self, degrees: isize) {
        self.degrees = (self.degrees + 360 - degrees) % 360;
    }

    fn turn_right(&mut self, degrees: isize) {
        self.degrees = (self.degrees + degrees) % 360;
    }

    fn current(&self) -> Direction {
        match self.degrees {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            _ => panic!("Error"),
        }
    }
}
