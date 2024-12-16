use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let (mut left, right) : (Vec<i32>, Vec<i32>)= io::stdin().lock().lines().map(|line| {
        line.unwrap().split_whitespace().map(|x| i32::from_str(x).unwrap()).collect::<Vec<i32>>()
    }).map(|x| (x[0], x[1])).unzip();
    left.sort();
    let mut right_map : HashMap<i32, i32> = HashMap::new();
    for num in right {
        *right_map.entry(num).or_insert(0) += 1;
    }
    let res = left.iter().fold(0, |acc, x| acc + if let Some(count) = right_map.get(x) {x * count} else { 0 });
    println!("{}", res);
}