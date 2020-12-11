use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let grid = create_grid(file_content);

    println!(
        "Result of puzzle 1: {}",
        count_occupied_seats(run_simulation(grid))
    );
}

fn create_grid(file_content: String) -> Grid {
    let lines: Vec<&str> = file_content.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let mut grid = vec![vec![Element::EmptySeat; cols]; rows];

    for i in 0..rows {
        let chars: Vec<char> = lines[i].chars().collect();
        for (j, c) in chars.iter().enumerate().take(cols) {
            if *c == '.' {
                grid[i][j] = Element::Floor;
            }
        }
    }

    grid
}

fn count_occupied_seats(grid: Grid) -> usize {
    grid.iter()
        .flat_map(|col| col.iter())
        .filter(|element| **element == Element::OccupiedSeat)
        .count()
}

fn run_simulation(grid: Grid) -> Grid {
    let mut current_grid = grid;

    loop {
        let next_grid = step(current_grid.clone());

        if next_grid == current_grid {
            break next_grid;
        }

        current_grid = next_grid;
    }
}

fn step(grid: Grid) -> Grid {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut next_grid = grid.clone();

    for i in 0..rows {
        for j in 0..cols {
            next_grid[i][j] = apply_rules(&grid, i, j);
        }
    }

    next_grid
}

fn apply_rules(grid: &Grid, row: usize, col: usize) -> Element {
    match grid[row][col] {
        Element::Floor => Element::Floor,
        Element::EmptySeat => {
            let elements = get_adjacent_elements(grid, row, col);
            if !elements
                .iter()
                .any(|element| *element == Element::OccupiedSeat)
            {
                Element::OccupiedSeat
            } else {
                Element::EmptySeat
            }
        }
        Element::OccupiedSeat => {
            let elements = get_adjacent_elements(grid, row, col);
            if elements
                .iter()
                .filter(|element| **element == Element::OccupiedSeat)
                .count()
                >= 4
            {
                Element::EmptySeat
            } else {
                Element::OccupiedSeat
            }
        }
    }
}

fn get_adjacent_elements(grid: &Grid, row: usize, col: usize) -> Vec<Element> {
    let mut elements = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    let irow: isize = row as isize;
    let icol: isize = col as isize;
    let all_possible_positions = vec![
        (irow - 1, icol),
        (irow - 1, icol + 1),
        (irow - 1, icol - 1),
        (irow, icol + 1),
        (irow, icol - 1),
        (irow + 1, icol),
        (irow + 1, icol + 1),
        (irow + 1, icol - 1),
    ];

    let possible_positions = all_possible_positions.iter().filter(|(row, col)| {
        *row >= 0 && *col >= 0 && *row < rows as isize && *col < cols as isize
    });
    for (row, col) in possible_positions {
        elements.push(grid[*row as usize][*col as usize]);
    }
    elements
}

type Grid = Vec<Vec<Element>>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Element {
    Floor,
    EmptySeat,
    OccupiedSeat,
}
