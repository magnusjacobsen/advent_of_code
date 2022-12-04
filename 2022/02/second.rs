use std::io::{self, BufRead};

// x loss
// y draw
// z win

fn main() {
    println!("{}", io::stdin().lock().lines().fold(0, |acc, current|  {
        acc + match current.unwrap().as_str() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        }
    }));
}