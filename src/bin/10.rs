use std::env;
use std::fs;

fn main() {
    const CHARGING_OUTLET_JOLTAGE: usize = 0;
    const BUILT_IN_JOLTAGE_ADAPTER_DIFF: usize = 3;

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
    output_joltages.push(output_joltages.last().unwrap() + BUILT_IN_JOLTAGE_ADAPTER_DIFF);

    let (one_jolt_diffs, three_jolt_diffs) = count_one_and_three_jolt_diffs(&output_joltages);
    let arrangements = count_arrangements(&output_joltages);

    println!("Result of puzzle 1: {}", one_jolt_diffs * three_jolt_diffs);
    println!("Result of puzzle 2: {}", arrangements);
}

fn count_one_and_three_jolt_diffs(output_joltages: &[usize]) -> (usize, usize) {
    let mut one_jolt_diffs = 0;
    let mut three_jolt_diffs = 0;

    for joltages in output_joltages.windows(2) {
        match joltages[1] - joltages[0] {
            1 => one_jolt_diffs += 1,
            3 => three_jolt_diffs += 1,
            _ => {}
        }
    }

    (one_jolt_diffs, three_jolt_diffs)
}

fn count_arrangements(output_joltages: &[usize]) -> usize {
    let mut arrangements = 1;
    let mut one_jolt_diff_counter: usize = 0;

    for joltages in output_joltages.windows(2) {
        if joltages[1] - joltages[0] == 1 {
            one_jolt_diff_counter += 1;
        } else {
            if one_jolt_diff_counter > 1 {
                arrangements *=
                    2_usize.pow(one_jolt_diff_counter as u32 - 2) + (one_jolt_diff_counter - 1);
            }
            one_jolt_diff_counter = 0;
        }
    }

    if one_jolt_diff_counter > 1 {
        arrangements *= 2_usize.pow(one_jolt_diff_counter as u32 - 2) + (one_jolt_diff_counter - 1);
    }

    arrangements
}
