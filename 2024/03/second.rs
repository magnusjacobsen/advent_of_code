use std::io::{self, BufRead};
use std::str::FromStr;

fn to_mul(s: &str) -> i32 {
    let nums = s.split(",").map(i32::from_str).map(Result::unwrap).collect::<Vec<i32>>();
    nums[0] * nums[1]
}

fn main() {
    let mut enabled = true;
    let res = io::stdin().lock().lines().map(Result::unwrap).fold(0, |acc, x| {
        if x == "do" {
            enabled = true;
            acc
        } else if x == "dont" {
            enabled = false;
            acc
        } else if enabled {
            acc + to_mul(&x)
        } else { acc }
    });
    println!("{}", res);
}