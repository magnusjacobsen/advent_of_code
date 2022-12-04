use std::io::{self, BufRead};

fn main() {
    println!("{}", io::stdin().lock().lines().fold(0, |acc, current|  {
        acc + match current.unwrap().as_str() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        }
    }));
}