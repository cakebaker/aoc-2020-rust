use std::env;
use std::fs;

fn main() {
    const RIGHT_SLOPE: usize = 3;
    const DEFAULT_DOWN_SLOPE: usize = 1;
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let map = create_map(file_content);

    println!(
        "Result of puzzle 1: {}",
        count_trees(&map, RIGHT_SLOPE, DEFAULT_DOWN_SLOPE)
    );

    let slopes = vec![
        (1, DEFAULT_DOWN_SLOPE),
        (3, DEFAULT_DOWN_SLOPE),
        (5, DEFAULT_DOWN_SLOPE),
        (7, DEFAULT_DOWN_SLOPE),
        (1, 2),
    ];
    let tree_counts = slopes
        .iter()
        .map(|(right_slope, down_slope)| count_trees(&map, *right_slope, *down_slope));

    println!("Result of puzzle 2: {}", tree_counts.product::<usize>());
}

fn count_trees(map: &[Vec<Element>], right_slope: usize, down_slope: usize) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut tree_counter = 0;
    let mut col = 0;

    for row in (0..rows).step_by(down_slope) {
        if map[row][col] == Element::Tree {
            tree_counter += 1;
        }
        col = (col + right_slope) % cols;
    }

    tree_counter
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
