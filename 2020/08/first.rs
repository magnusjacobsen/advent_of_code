use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
}

fn parse_input() -> Vec<(Instruction, i32)> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut instructions: Vec<(Instruction, i32)> = vec![];
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" {
            continue;
        }
        let splitted: Vec<&str> = tmp.split(" ").collect();
        let instruction: Instruction = match splitted[0] {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc,
            "jmp" => Instruction::Jmp,
            _     => Instruction::Nop,
        };
        let value = i32::from_str(splitted[1]).unwrap();
        instructions.push((instruction, value));
    }
    instructions
}

fn run_instructions(instructions: Vec<(Instruction, i32)>) -> i32 {
    let mut accumuluator: i32 = 0;
    let mut unvisited = vec![true; instructions.len()];
    let mut index: i32 = 0;
    while unvisited[index as usize] {
        let (instruction, value) = instructions[index as usize];
        unvisited[index as usize] = false;
        match instruction {
            Instruction::Nop => {
                index += 1;
            }
            Instruction::Acc => {
                accumuluator += value;
                index += 1;
            } 
            Instruction::Jmp => {
                index += value;
            }
        }
    }
    accumuluator
}

fn main() {
    let instructions = parse_input();
    let result = run_instructions(instructions);
    println!("{}", result);
}