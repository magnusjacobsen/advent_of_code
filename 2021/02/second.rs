use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines().map(|x| {
            let line = x.unwrap();
            let splt = line.split(" ").collect::<Vec<_>>();
            (splt[0].to_string(), usize::from_str(splt[1]).unwrap())
        }).collect::<Vec<(String, usize)>>();
    let mut hori: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for (ins, x) in instructions {
        let amount = x as i32;
        if ins == "forward".to_string() {
            hori += amount;
            depth += aim * amount;
        } else if ins == "down".to_string() {
            aim += amount;
        } else if ins == "up".to_string() {
            aim -= amount;
        }
    }
    println!("hori: {}, depth: {}, mult: {}", hori, depth, hori * depth);
}
