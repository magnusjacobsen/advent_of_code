use std::io::{self, BufRead};

fn main() {
    let res = io::stdin().lock().lines().fold(0, |acc, x| {
        let mut line = x.unwrap().split_whitespace().map(|y| y.parse::<i32>().unwrap()).filter(|x| x > &0).collect::<Vec<_>>();
        line.sort();
        if line.len() == 3 && line[0] + line[1] > line[2] { acc + 1 } else { acc }
    });
    println!("{}", res);
}