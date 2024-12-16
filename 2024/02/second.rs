use std::io::{self, BufRead};
use std::str::FromStr;

fn get_diff_list(vec: &Vec<i32>, ignore: usize) -> Vec<i32> {
    if vec.len() < 2 { 
        return vec![]; 
    }
    let mut out = vec![];
    let last = if ignore == vec.len() - 1 {vec.len() - 2} else {vec.len() - 1};
    for i in 0..last {
        if i == ignore {
            continue;
        }
        let next = if ignore == i + 1 {i + 2} else { i + 1};
        let diff = vec[i] - vec[next];
        out.push(diff);
    }
    out
}

fn is_safe(vec: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while i < vec.len() && res == 0 {
        let diff_list = get_diff_list(&vec, i);
        let mut all_dec = true;
        let mut all_acc = true;
        'a: for j in 0..diff_list.len() {
            let level_diff = diff_list[j];
            if all_acc && level_diff > 0 && level_diff < 4 {
                all_dec = false;
            } else if all_dec && level_diff < 0 && level_diff > -4 {
                all_acc = false;
            } else {
                all_dec = false;
                all_acc = false;
                break 'a;
            }
        }
        if all_dec || all_acc {
            res = 1;
        }
        i += 1;
    }
    res
}

fn main() {
    let result = io::stdin().lock().lines()
        .fold(0, |acc, x|
            acc + is_safe(
                x.unwrap()
                .split_whitespace()
                .map(i32::from_str)
                .map(Result::unwrap)
                .collect::<Vec<i32>>()
            )
        );
    println!("{}", result);
}