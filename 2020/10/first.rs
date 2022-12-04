use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> Vec<usize> {
    io::stdin().lock()
        .lines()
        .filter(|x| x.as_ref().unwrap() != "")
        .map(|x| usize::from_str(&x.unwrap()).unwrap())
        .collect()
}

fn find_difference(mut adapters: Vec<usize>) -> usize {
    adapters.sort();
    let mut ones = 0;
    let mut threes = 1;
    let mut prev = 0;
    for a in adapters {
        match a - prev {
            1 => ones += 1,
            3 => threes += 1,
            _ => {},
        }
        prev = a;
    }
    ones * threes
}

fn main() {
    let adapters = parse_input();
    let difference = find_difference(adapters);
    println!("{}", difference);
}