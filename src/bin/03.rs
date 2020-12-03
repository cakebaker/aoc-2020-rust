use std::env;
use std::fs;

fn main() {
    const RIGHT_SLOPE: usize = 3;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let map = create_map(file_content);

    let rows = map.len();
    let cols = map[0].len();
    let mut tree_counter = 0;
    let mut col = 0;

    for row in 0..rows {
        if map[row][col] == Element::Tree {
            tree_counter += 1;
        }
        col = (col + RIGHT_SLOPE) % cols;
    }

    println!("Result of puzzle 1: {}", tree_counter);
}

fn create_map(file_content: String) -> Vec<Vec<Element>> {
    let lines: Vec<&str> = file_content.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let mut map = vec![vec![Element::OpenSquare; cols]; rows];

    for i in 0..rows {
        let chars: Vec<char> = lines[i].chars().collect();
        for (j, c) in chars.iter().enumerate().take(cols) {
            if *c == '#' {
                map[i][j] = Element::Tree;
            }
        }
    }
    map
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
    OpenSquare,
    Tree,
}
