use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let lines = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
    for i in 0..std::cmp::max(lines.len() - 14, 0) {
        if lines[i..i + 14].iter().collect::<HashSet<_>>().len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }
}