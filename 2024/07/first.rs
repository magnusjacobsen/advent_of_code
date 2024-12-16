use std::io::{self, BufRead};
use std::str::FromStr;

fn solve(inc: i64, result: i64, rest: &[i64]) -> i64 {
    if inc > result {
        return inc;
    }
    if rest.len() == 0 {
        return inc;
    }
    if result == solve(inc * rest[0], result, &rest[1..]) {
        return result;
    }
    return solve(inc + rest[0], result, &rest[1..]);
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap).collect::<Vec<_>>();
    let mut total_result = 0;
    for line in lines {
        let splitted = line.split(": ").collect::<Vec<_>>();
        let result = i64::from_str(splitted[0]).unwrap();
        let numbers = splitted[1].split(" ").map(i64::from_str).map(Result::unwrap).collect::<Vec<i64>>();
        if result == solve(numbers[0], result, &numbers[1..]) {
            total_result += result;
        }
    }
    println!("{}", total_result);
}
