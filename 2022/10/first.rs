use std::io::BufRead;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn get_num_cycles(ins: &Instruction) -> u32 {
    match ins {
        Instruction::Noop       => 1,
        Instruction::Addx(_)    => 2,
    }
}

fn main() {
    let mut register_x = 1;
    let mut cycle_count = 0;
    let mut weird_sum = 0;
    let mut instructions = std::io::stdin().lock().lines().map(|line| {
        let line2 = line.unwrap();
        if line2 == "noop" {
            Instruction::Noop
        } else {
            let splt = line2.split(" ").collect::<Vec<_>>();
            let val: i32 = splt[1].parse().unwrap();
            Instruction::Addx(val)
        }
    }).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>();

    while let Some(instruction) = instructions.pop() {
        let mut instruction_cycles = get_num_cycles(&instruction);
        while instruction_cycles > 0 {
            cycle_count += 1;
            instruction_cycles -= 1;
            if cycle_count == 20 || (cycle_count - 20) % 40 == 0 {
                weird_sum += cycle_count * register_x;
            }
        }
        match instruction {
            Instruction::Noop       => (),
            Instruction::Addx(val)  => register_x += val,
        }
    }
    println!("{}", weird_sum);
}