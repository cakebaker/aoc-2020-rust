use std::env;
use std::fs;

fn main() {
    const INITIAL_DIRECTION: isize = 90;
    const INITIAL_X: isize = 0;
    const INITIAL_Y: isize = 0;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let actions: Vec<Action> = file_content.lines().map(|line| parse(line)).collect();

    let end_position = get_location(
        &actions,
        Position {
            x: INITIAL_X,
            y: INITIAL_Y,
        },
        FacingDirection::new(INITIAL_DIRECTION),
    );

    println!(
        "Result of puzzle 1: {}",
        end_position.x.abs() + end_position.y.abs()
    );
}

fn get_location(
    actions: &[Action],
    start_position: Position,
    mut direction: FacingDirection,
) -> Position {
    let mut x = start_position.x;
    let mut y = start_position.y;

    for action in actions {
        match action {
            Action::North(v) => y += v,
            Action::South(v) => y -= v,
            Action::East(v) => x += v,
            Action::West(v) => x -= v,
            Action::Left(v) => direction.turn_left(*v),
            Action::Right(v) => direction.turn_right(*v),
            Action::Forward(v) => match direction.current() {
                Direction::North => y += v,
                Direction::South => y -= v,
                Direction::East => x += v,
                Direction::West => x -= v,
            },
        };
    }

    Position { x, y }
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
        "F" => Action::Forward(value),
        _ => panic!("Unexpected char!"),
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

struct Position {
    x: isize,
    y: isize,
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
