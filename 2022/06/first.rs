use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let lines = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
    for i in 0..std::cmp::max(lines.len() - 4, 0) {
        if lines[i..i + 4].iter().collect::<HashSet<_>>().len() == 4 {
            println!("{}", i + 4);
            break;
        }
    }
}