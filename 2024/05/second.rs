use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn is_correct(update: &Vec<i32>, verboten: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut seen = vec![];
    update.iter().all(|x| {
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
    })
}

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
    let mut incorrect = vec![];
    while i < lines.len() {
        let update = lines[i].split(",").map(i32::from_str).map(Result::unwrap).collect::<Vec<_>>();
        i += 1;
        if !is_correct(&update, &verboten) {
            incorrect.push(update);
        }
    }
    let mut middle_numbers = vec![];
    for update in incorrect {
        let mut current = update.clone();
        while !is_correct(&current, &verboten) {
            for i in 1..current.len() {
                let a = current[i - 1];
                let b = current[i];
                if let Some(set) = verboten.get(&b) {
                    if set.contains(&a) {
                        current[i - 1] = b;
                        current[i] = a;
                    }
                }
            }
        }
        let middle_number = current[current.len() / 2];
        middle_numbers.push(middle_number);
    }

    let sum = middle_numbers.iter().sum::<i32>();
    println!("{}", sum);
}