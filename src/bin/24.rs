use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    const MAX_DAYS: usize = 100;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let mut black_tiles = get_black_tiles(parse(file_content));

    println!("Result of puzzle 1: {}", black_tiles.len());

    for _ in 1..=MAX_DAYS {
        black_tiles = daily_flip(&black_tiles);
    }

    println!("Result of puzzle 2: {}", black_tiles.len());
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

fn daily_flip(current_black_tiles: &HashSet<Position>) -> HashSet<Position> {
    let mut next_black_tiles = HashSet::new();
    let mut potential_black_tiles = HashSet::new();

    for position in current_black_tiles {
        let count = count_neighbors(&current_black_tiles, *position);

        if count == 1 || count == 2 {
            next_black_tiles.insert(*position);
        }

        potential_black_tiles.extend(get_neighbor_positions(*position));
    }

    for position in potential_black_tiles {
        if count_neighbors(current_black_tiles, position) == 2 {
            next_black_tiles.insert(position);
        }
    }

    next_black_tiles
}

fn count_neighbors(positions: &HashSet<Position>, position: Position) -> usize {
    positions
        .iter()
        .filter(|&p| is_neighbor(*p, position))
        .count()
}

fn get_neighbor_positions(position: Position) -> Vec<Position> {
    let neighbor_positions = vec![(2, 0), (-2, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    neighbor_positions
        .iter()
        .map(|(x, y)| Position {
            x: position.x + x,
            y: position.y + y,
        })
        .collect()
}

fn is_neighbor(a: Position, b: Position) -> bool {
    a.x + 2 == b.x && a.y == b.y
        || a.x - 2 == b.x && a.y == b.y
        || a.x + 1 == b.x && a.y + 1 == b.y
        || a.x + 1 == b.x && a.y - 1 == b.y
        || a.x - 1 == b.x && a.y + 1 == b.y
        || a.x - 1 == b.x && a.y - 1 == b.y
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
