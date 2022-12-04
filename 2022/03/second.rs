use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines().map(|line| line.unwrap().chars().map(|c| if c.is_lowercase() { c as u32 - 96 } else { c as u32 - 'A' as u32 + 27 } as usize).collect()).collect::<Vec<std::collections::HashSet<_>>>();
    println!("{}", (0..lines.len() / 3).map(|i| i * 3).fold(0, |acc, i| acc + lines[i].intersection(&lines[i + 1]).map(|x| *x).collect::<std::collections::HashSet<_>>().intersection(&lines[i + 2]).next().unwrap().clone() ));
}