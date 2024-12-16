use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let nums = io::stdin().lock().take(1).expect("there should be a single line")
        .chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<i32>>();
    println!("{:?}", nums);
}