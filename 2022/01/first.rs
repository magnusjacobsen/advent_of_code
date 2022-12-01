use std::io::{self, BufRead};

fn main() {
    println!("{}", io::stdin().lock().lines().fold([0, 0], |acc, current| match current.unwrap().as_str() {"" => [*acc.iter().max().unwrap(), 0], num => [acc[0], u32::from_str_radix(&num, 10).unwrap() + acc[1]], }).iter().max().unwrap());
}