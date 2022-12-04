use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> Vec<i64> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let numbers: Vec<i64> = lines
        .filter(|x| x.as_ref().unwrap() != "")
        .map(|x| i64::from_str(&x.unwrap()).unwrap())
        .collect();
    numbers
}

fn find_weakness(numbers: Vec<i64>) -> (usize, i64) {
    let mut index = 25;
    'a: while index < numbers.len() {
        let num = numbers[index];
        for i in index - 25..index {
            let must_match = num - numbers[i];
            for j in i..index {
                if must_match == numbers[j] {
                    index += 1;
                    continue 'a;
                }
            }
        }
        return (index, num);
    }
    (0, 0)
}

fn main() {
    let numbers = parse_input();
    let (index, number) = find_weakness(numbers);
    println!("index: {}, number: {}", index, number);
}