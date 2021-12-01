use std::io::{self,BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    
    let mut numbers: Vec<u32> = lines.map(|x| u32::from_str(&x.unwrap()).unwrap()).collect();
    numbers.sort();
    'outer: for i in (0..numbers.len()).rev() {
        'inner: for j in 0..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                println!("{} * {} = {}", numbers[i], numbers[j], numbers[i] * numbers[j]);
                break 'outer;
            } else if numbers[i] + numbers[j] > 2020 {
                break 'inner;
            }
        }
    }
}