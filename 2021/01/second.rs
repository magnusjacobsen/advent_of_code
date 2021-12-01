use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let numbers = lines.map(|x| i32::from_str(&x.unwrap()).unwrap()).collect::<Vec<i32>>();
    let mut sliding_window = vec![];

    for i in 0..numbers.len() - 2 {
        let val = numbers[i] + numbers[i + 1] + numbers[i + 2];
        sliding_window.push(val);
    }

    let mut num_increases = 0;
    for i in 1..sliding_window.len() {
        if sliding_window[i - 1] < sliding_window[i] {
            num_increases += 1;
        }
    }    

    println!("{:?}", num_increases);
}
