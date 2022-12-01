use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let(a, b) = io::stdin().lock().lines().fold((0, 0), |acc, current| match current.unwrap().as_str() {
        "" => (max(acc.0, acc.1), 0),
        num => (acc.0, u32::from_str_radix(&num, 10).unwrap() + acc.1),
    });
    println!("{}", max(a, b));
}