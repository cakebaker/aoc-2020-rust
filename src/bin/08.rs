use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_content =
        fs::read_to_string(filename).expect("Something went wrong reading the file!");

    println!(
        "Result of puzzle 1: {}",
        run_instructions(get_instructions(file_content))
    );
}

fn get_instructions(file_content: String) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for file in file_content.lines() {
        let v: Vec<&str> = file.split(' ').collect();
        let offset = v[1].parse().unwrap();

        let instruction = match v[0] {
            "acc" => Instruction::Acc(offset),
            "jmp" => Instruction::Jmp(offset),
            _ => Instruction::Nop(offset),
        };
        instructions.push(instruction);
    }
    instructions
}

fn run_instructions(instructions: Vec<Instruction>) -> isize {
    let mut accumulator = 0;
    let mut next_index = 0;
    let mut executed_instructions: Vec<usize> = Vec::new();

    loop {
        if !executed_instructions.contains(&next_index) {
            executed_instructions.push(next_index);
            let (i, increase) = run_instruction(&instructions, next_index);
            accumulator += increase;
            next_index = i;
        } else {
            break accumulator;
        }
    }
}

fn run_instruction(instructions: &[Instruction], index: usize) -> (usize, isize) {
    let instruction = &instructions[index];

    match instruction {
        Instruction::Acc(i) => (index + 1, *i),
        Instruction::Jmp(i) => (((index as isize) + *i) as usize, 0),
        Instruction::Nop(_) => (index + 1, 0),
    }
}

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}
