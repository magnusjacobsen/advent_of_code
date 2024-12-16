use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let res = io::stdin().lock().lines().fold(0, |acc, x| {
        let splitted = x.unwrap().split(",").map(i32::from_str).map(Result::unwrap).collect::<Vec<i32>>();
        acc + splitted[0] * splitted[1]
    });
    println!("{}", res);
}