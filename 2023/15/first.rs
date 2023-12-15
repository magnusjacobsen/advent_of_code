use std::io::BufRead;

fn main() {
    println!("{}", &std::io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>()[0].split(",").fold(0, |sum,elem| sum + elem.chars().fold(0, |current, c| ((current + c as u64) * 17) % 256)));
}