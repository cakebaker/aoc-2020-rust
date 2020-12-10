use std::env;
use std::fs;

fn main() {
    const CHARGING_OUTLET_JOLTAGE: usize = 0;

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let mut output_joltages: Vec<usize> = file_content
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    output_joltages.push(CHARGING_OUTLET_JOLTAGE);
    output_joltages.sort_unstable();

    let mut one_jolt_diffs = 0;
    let mut three_jolt_diffs = 1; // built-in joltage adapter always has a 3 jolt difference

    for joltage_pair in output_joltages.windows(2) {
        match joltage_pair[1] - joltage_pair[0] {
            1 => one_jolt_diffs += 1,
            3 => three_jolt_diffs += 1,
            _ => {}
        }
    }

    println!("Result of puzzle 1: {}", one_jolt_diffs * three_jolt_diffs);
}
