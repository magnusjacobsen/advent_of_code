use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> Vec<i64> {
    io::stdin()
        .lock()
        .lines()
        .filter(|x| x.as_ref().unwrap() != "")
        .map(|x| i64::from_str(&x.unwrap()).unwrap())
        .collect()
}

fn find_weakness(numbers: Vec<i64>) -> (Vec<i64>, usize) {
    'a: for i in 25..numbers.len() {
        for j in i-25..i {
            for k in j+1..i {
                if numbers[i] - numbers[j] == numbers[k] {
                    continue 'a;
                }
            }
        }
        return (numbers, i);
    }
    (vec![], 0)
}

fn exploit_weakness((numbers, target): (Vec<i64>, usize)) -> i64 {
    'a: for i in 0..numbers.len() {
        'b: for j in i..numbers.len() {
            match numbers[i..j + 1].iter().sum::<i64>() {
                sum if sum < numbers[target] => continue 'b,
                sum if sum > numbers[target] => continue 'a,
                _ => return 
                    numbers[i..j + 1].iter().min().unwrap() + 
                    numbers[i..j + 1].iter().max().unwrap(),
            }
        }
    }
    return 0
} 

fn main() {
    println!("exploit: {}", exploit_weakness(find_weakness(parse_input())));
}