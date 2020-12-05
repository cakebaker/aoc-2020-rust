use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut seat_ids = get_seat_ids(lines);
    seat_ids.sort_unstable();
    let highest_seat_id = seat_ids[seat_ids.len() - 1];

    println!("Result of puzzle 1: {}", highest_seat_id);
    println!(
        "Result of puzzle 2: {}",
        find_free_seat_id(seat_ids).unwrap()
    );
}

fn find_free_seat_id(seat_ids: Vec<u32>) -> Option<u32> {
    let mut previous_id = 0;

    for seat_id in seat_ids {
        if (seat_id - previous_id) == 2 {
            return Some(seat_id - 1);
        }
        previous_id = seat_id;
    }
    None
}

fn get_seat_ids(lines: Vec<&str>) -> Vec<u32> {
    let mut seat_ids: Vec<u32> = Vec::new();
    for line in lines {
        let s: String = line
            .chars()
            .map(|c| match c {
                'F' | 'L' => '0',
                _ => '1',
            })
            .collect();
        let (binary_row, binary_column) = s.split_at(7);
        let row = u32::from_str_radix(binary_row, 2).unwrap();
        let column = u32::from_str_radix(binary_column, 2).unwrap();

        let seat_id = 8 * row + column;
        seat_ids.push(seat_id);
    }
    seat_ids
}
