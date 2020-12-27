use std::env;
use std::fs;

fn main() {
    const INITIAL_DIRECTION: isize = 90;
    const INITIAL_SHIP_X: isize = 0;
    const INITIAL_SHIP_Y: isize = 0;
    const INITIAL_WAYPOINT_X: isize = 10;
    const INITIAL_WAYPOINT_Y: isize = 1;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let actions: Vec<Action> = file_content.lines().map(|line| parse(line)).collect();
    let initial_ship_position = Position {
        x: INITIAL_SHIP_X,
        y: INITIAL_SHIP_Y,
    };

    let end_position = get_location(
        &actions,
        initial_ship_position,
        FacingDirection::new(INITIAL_DIRECTION),
    );

    let initial_waypoint_position = Position {
        x: INITIAL_WAYPOINT_X,
        y: INITIAL_WAYPOINT_Y,
    };
    let ship_position =
        get_location_using_waypoint(&actions, initial_ship_position, initial_waypoint_position);

    println!(
        "Result of puzzle 1: {}",
        end_position.x.abs() + end_position.y.abs()
    );

    println!(
        "Result of puzzle 2: {}",
        ship_position.x.abs() + ship_position.y.abs()
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

fn get_location_using_waypoint(
    actions: &[Action],
    mut ship: Position,
    mut waypoint: Position,
) -> Position {
    for action in actions {
        match action {
            Action::North(v) => waypoint.y += v,
            Action::South(v) => waypoint.y -= v,
            Action::East(v) => waypoint.x += v,
            Action::West(v) => waypoint.x -= v,
            Action::Left(v) => match v {
                90 => {
                    let temp = waypoint.x;
                    waypoint.x = 0 - waypoint.y;
                    waypoint.y = temp;
                }
                180 => {
                    waypoint.x = 0 - waypoint.x;
                    waypoint.y = 0 - waypoint.y;
                }
                270 => {
                    let temp = waypoint.x;
                    waypoint.x = waypoint.y;
                    waypoint.y = 0 - temp;
                }
                _ => panic!("Error"),
            },
            Action::Right(v) => match v {
                90 => {
                    let temp = waypoint.x;
                    waypoint.x = waypoint.y;
                    waypoint.y = 0 - temp;
                }
                180 => {
                    waypoint.x = 0 - waypoint.x;
                    waypoint.y = 0 - waypoint.y;
                }
                270 => {
                    let temp = waypoint.x;
                    waypoint.x = 0 - waypoint.y;
                    waypoint.y = temp;
                }
                _ => panic!("Error"),
            },
            Action::Forward(v) => {
                ship.x += v * waypoint.x;
                ship.y += v * waypoint.y;
            }
        }
    }

    ship
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

#[derive(Clone, Copy)]
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
