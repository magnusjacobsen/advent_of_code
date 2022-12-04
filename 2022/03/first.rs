use std::io::{self, BufRead};

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn main() {
    println!("{}", io::stdin().lock().lines().fold(0, |acc, line|  {
        let mut first_compartment = [0; 53];
        let chars = line.unwrap().chars().collect::<Vec<_>>();
        let len = chars.len();
        for i in 0..len {
            let priority = get_priority(chars[i]) as usize;
            if i < len / 2 {
                first_compartment[priority] = 1;
            } else {
                if first_compartment[priority] == 1 {
                    return acc + priority;
                }
            }
        }
        acc
    }));
}