use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let (mut left, mut right) : (Vec<i32>, Vec<i32>)= io::stdin().lock().lines().map(|line| {
        line.unwrap().split_whitespace().map(|x| i32::from_str(x).unwrap()).collect::<Vec<i32>>()
    }).map(|x| (x[0], x[1])).unzip();
    left.sort();
    right.sort();
    let res = left.iter().zip(right.iter()).fold(0, |acc, x| acc + (x.0 - x.1).abs());
    println!("{}", res);
}