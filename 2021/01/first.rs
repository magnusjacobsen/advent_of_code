use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let nums: Vec<i32> = stdin.lock().lines().map(|x| i32::from_str(&x.unwrap()).unwrap()).collect();
    println!("{}", (0..nums.len() - 1).map(|i| (nums[i] < nums[i + 1]) as i32).sum::<i32>());
}