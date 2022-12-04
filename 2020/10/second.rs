use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> Vec<usize> {
    io::stdin().lock()
        .lines()
        .filter(|x| x.as_ref().unwrap() != "")
        .map(|x| usize::from_str(&x.unwrap()).unwrap())
        .collect()
}

fn count_arrangements(index: usize, adapters: &Vec<usize>, memo: &mut Vec<i64>) -> i64 {
    if index == adapters.len() - 1 {
        return 1;
    }
    let current = adapters[index];
    let mut count = 0;
    for i in index + 1..index + 4 {
        if i < adapters.len() {
            if adapters[i] < current + 4 {
                count += if memo[i] > -1 {
                    memo[i]
                } else {
                    count_arrangements(i, adapters, memo)
                }
            }
        }
    }
    memo[index] = count;
    return count;
}

fn main() {
    let mut adapters = parse_input();
    adapters.sort();
    adapters.insert(0,0);
    adapters.push(adapters[adapters.len() - 1] + 3);
    let mut memo: Vec<i64> = vec![-1; adapters.len()];
    let count = count_arrangements(0, &adapters, &mut memo);
    println!("{}", count);
}