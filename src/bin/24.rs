use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let black_tiles = get_black_tiles(parse(file_content));

    println!("Result of puzzle 1: {}", black_tiles.len());
}

fn get_black_tiles(tiles: Vec<Vec<Direction>>) -> HashSet<Position> {
    let identified_tiles: Vec<Position> = tiles.iter().map(|tile| identify_tile(tile)).collect();
    let mut black_tiles = HashSet::new();

    for identified_tile in identified_tiles {
        if black_tiles.contains(&identified_tile) {
            black_tiles.remove(&identified_tile);
        } else {
            black_tiles.insert(identified_tile);
        }
    }

    black_tiles
}

fn identify_tile(directions: &[Direction]) -> Position {
    let mut x = 0;
    let mut y = 0;

    for direction in directions {
        match direction {
            Direction::East => x += 2,
            Direction::West => x -= 2,
            Direction::SouthEast => {
                y -= 1;
                x += 1;
            }
            Direction::SouthWest => {
                y -= 1;
                x -= 1;
            }
            Direction::NorthEast => {
                y += 1;
                x += 1;
            }
            Direction::NorthWest => {
                y += 1;
                x -= 1;
            }
        }
    }

    Position { x, y }
}

fn parse(file_content: String) -> Vec<Vec<Direction>> {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut tiles = Vec::new();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut directions = Vec::new();
        let mut i = 0;

        while i < chars.len() {
            let direction = match chars[i] {
                'e' => Direction::East,
                'w' => Direction::West,
                's' => {
                    i += 1;
                    match chars[i] {
                        'e' => Direction::SouthEast,
                        'w' => Direction::SouthWest,
                        _ => panic!("Unexpected character detected!"),
                    }
                }
                'n' => {
                    i += 1;
                    match chars[i] {
                        'e' => Direction::NorthEast,
                        'w' => Direction::NorthWest,
                        _ => panic!("Unexpected character detected!"),
                    }
                }
                _ => panic!("Unexpected character detected!"),
            };

            directions.push(direction);
            i += 1;
        }
        tiles.push(directions);
    }

    tiles
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
