use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    println!("{}", (0..80).fold(io::stdin().lock().lines().next().unwrap().unwrap().split(',').map(|x| usize::from_str(x).unwrap() ).fold(vec![0; 9], |mut acc, x| { acc[x] += 1; acc }), |acc, _| (0..9).map(|i| if i == 6 { acc[i + 1] + acc[0] } else if i == 8 { acc[0] }  else { acc[i + 1] }).collect() ).iter().sum::<u128>());
}