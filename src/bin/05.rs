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
    let highest_seat_id = seat_ids.last().unwrap();

    println!("Result of puzzle 1: {}", highest_seat_id);
    println!(
        "Result of puzzle 2: {}",
        find_free_seat_id(seat_ids).unwrap()
    );
}

fn find_free_seat_id(seat_ids: Vec<u32>) -> Option<u32> {
    for pair in seat_ids.windows(2) {
        if pair[0] + 1 != pair[1] {
            return Some(pair[0] + 1);
        }
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
        let row = u32::from_str_radix(&s[..7], 2).unwrap();
        let column = u32::from_str_radix(&s[7..], 2).unwrap();

        let seat_id = 8 * row + column;
        seat_ids.push(seat_id);
    }
    seat_ids
}
