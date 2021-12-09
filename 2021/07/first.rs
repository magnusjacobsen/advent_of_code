use std::io::{self, BufRead};

fn main() {
    let vals = io::stdin()
        .lock()
        .lines()
        .next().unwrap().unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let min_val = vals.iter().min().unwrap();
    let max_val = vals.iter().max().unwrap();
    let mut min_fuel = std::i32::MAX;
    let mut min_candidate = 0;
    for i in *min_val..*max_val {
        let mut sum = 0;
        for val in &vals {
            sum += (i - *val).abs();
        }
        if sum < min_fuel {
            min_fuel = sum;
            min_candidate = i;
        }
    }
    println!("min_fuel: {}, position: {}", min_fuel, min_candidate);
}