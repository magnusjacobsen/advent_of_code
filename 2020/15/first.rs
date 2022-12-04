use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

fn parse_input() -> Vec<usize> {
    io::stdin()
        .lock()
        .lines()
        .next().unwrap().unwrap()
        .split(",")
        .map(|x| usize::from_str(x).unwrap())
        .collect()
}

fn game(numbers: Vec<usize>) -> usize{
    let mut last_spoken: HashMap<usize, usize> = HashMap::new();
    let mut prev: (usize, bool) = (0, true); // number, first time spoken?
    let mut turn = 0;
    for i in 0..numbers.len() {
        turn += 1;
        last_spoken.insert(numbers[i], turn);
        prev = (numbers[i], true);
    }
    while turn < 2020 {
        if prev.1 {
            last_spoken.insert(prev.0, turn); // inserting the previous number to last_spoken
            prev = (0, !last_spoken.contains_key(&0));
        } else {
            let apart = turn - last_spoken.get(&prev.0).unwrap();
            last_spoken.insert(prev.0, turn);
            prev = (apart, !last_spoken.contains_key(&apart));
        }
        turn += 1;
    }
    prev.0
}

fn main() {
    let input = parse_input();
    let result = game(input);
    println!("{}", result);
}