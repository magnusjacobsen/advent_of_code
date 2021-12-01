use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let numbers = lines.map(|x| i32::from_str(&x.unwrap()).unwrap()).collect::<Vec<i32>>();
    let mut num_increases = 0;
    for i in 1..numbers.len() {
        if numbers[i - 1] < numbers[i] {
            num_increases += 1;
        }
    }    

    println!("{:?}", num_increases);
}
