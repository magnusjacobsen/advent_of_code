use std::io::{self, BufRead};
use std::str::FromStr;

fn diff_list(vec: Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 { 
        return vec![]; 
    }
    let mut out = vec![];
    for i in 0..vec.len() - 1 {
        let diff = vec[i] - vec[i + 1];
        out.push(diff);
    }
    out
}

fn main() {
    let result = io::stdin().lock().lines()
        .fold(0, |accum, x| {
            let reports = diff_list(
                x.unwrap()
                .split_whitespace()
                .map(i32::from_str)
                .map(Result::unwrap)
                .collect::<Vec<i32>>()
            );
            if reports.iter().all(|y| y > &0 && y < &4) {accum + 1}
            else if reports.iter().all(|y| y < &0 && y > &-4) {accum + 1}
            else {accum}
        });
    println!("{}", result);
}