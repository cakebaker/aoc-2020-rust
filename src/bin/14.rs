use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    let program = parse(file_content);
    let memory = run_program(program);

    println!("Result of puzzle 1: {}", memory.values().sum::<usize>());
}

fn run_program(program: Vec<(String, Vec<(usize, usize)>)>) -> HashMap<usize, usize> {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for (mask, memory_instructions) in program {
        let (bit_or_mask, bit_and_mask) = create_bitmasks(mask);

        for (address, value) in memory_instructions {
            let mut masked_value = value;
            masked_value |= bit_or_mask;
            masked_value &= bit_and_mask;

            memory.insert(address, masked_value);
        }
    }

    memory
}

fn create_bitmasks(mask: String) -> (usize, usize) {
    let bit_or = &mask.replace("X", "0");
    let bit_and = &mask.replace("X", "1");

    (
        usize::from_str_radix(bit_or, 2).unwrap(),
        usize::from_str_radix(bit_and, 2).unwrap(),
    )
}

fn parse(file_content: String) -> Vec<(String, Vec<(usize, usize)>)> {
    let lines: Vec<&str> = file_content.lines().collect();

    let mut program = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let mut memory_instructions: Vec<(usize, usize)> = Vec::new();
        let (_, mask) = lines[i].split_at(7);
        i += 1;

        while i < lines.len() && lines[i].starts_with("mem") {
            let elements: Vec<&str> = lines[i].split(|c| c == ' ').collect();
            let address = elements[0][4..(elements[0].len() - 1)].parse().unwrap();
            let value = elements[2].parse().unwrap();

            memory_instructions.push((address, value));
            i += 1;
        }

        program.push((mask.to_string(), memory_instructions));
    }

    program
}
