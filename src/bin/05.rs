use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let lines: Vec<&str> = file_content.lines().collect();

    let mut highest_seat_id = 0;
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

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    println!("Result of puzzle 1: {}", highest_seat_id);
}
