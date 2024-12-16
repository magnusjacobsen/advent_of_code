use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap).collect::<Vec<_>>();
    let mut i = 0;
    let mut verboten : HashMap<i32, HashSet<i32>> = HashMap::new();
    loop {
        let line = lines[i].clone();
        i += 1;
        if line == "" { break; }
        let splitted = line.split("|").map(i32::from_str).map(Result::unwrap).collect::<Vec<_>>();
        let a = splitted[0];
        let b = splitted[1];
        (*verboten.entry(a).or_insert(HashSet::new())).insert(b);
    }
    let mut middle_numbers = vec![];
    while i < lines.len() {
        let update = lines[i].split(",").map(i32::from_str).map(Result::unwrap).collect::<Vec<_>>();
        i += 1;
        let mut seen = vec![];
        if update.iter().all(|x| {
            if let Some(set) = verboten.get(x) {
                if !seen.iter().any(|y| set.contains(y)) {
                    seen.push(*x);
                    true
                } else {
                    false
                }
            } else {
                seen.push(*x);
                true
            }
        }) {
            let middle_number = update[update.len() / 2];
            middle_numbers.push(middle_number);
        }
    }
    let sum = middle_numbers.iter().sum::<i32>();
    println!("{}", sum);
}