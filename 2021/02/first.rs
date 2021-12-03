use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines().map(|x| {
            let line = x.unwrap();
            let splt = line.split(" ").collect::<Vec<_>>();
            (splt[0].to_string(), usize::from_str(splt[1]).unwrap())
        }).collect::<Vec<(String, usize)>>();
    let mut hori = 0;
    let mut depth = 0;

    for (ins, amount) in instructions {
        if ins == "forward".to_string() {
            hori += amount;
        } else if ins == "down".to_string() {
            depth += amount;
        } else if ins == "up".to_string() {
            depth -= amount;
        } else if ins == "backward".to_string() {
            hori -= amount;
        }
    }
    println!("hori: {}, depth: {}, mult: {}", hori, depth, hori * depth);
}
