use std::io::{self,BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut count = 0;

    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" {
            continue;
        }
        let split: Vec<&str> = tmp.split(" ").collect();
        let min = usize::from_str(split[0].split("-").next().unwrap()).unwrap();
        let max = usize::from_str(split[0].split("-").take(2).last().unwrap()).unwrap();
        let character: &str = &split[1][0..1];
        let password: &str = split[2];
        let c = password.matches(character).count();
        count = if c >= min && c <= max { count + 1 }
                else { count }
    }
    println!("{:?}", count);
}