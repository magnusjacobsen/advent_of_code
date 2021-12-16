#![feature(stdin_forwarders)]
fn main() {
    let mut vals = std::io::stdin().lines().map(|x| x.unwrap().chars().fold((String::new(), false), |(mut open, is_faulty), c| if c == '(' || c == '[' || c == '{' || c == '<' { open.push(c); (open, is_faulty || false) } else if c as i32 - open.chars().last().unwrap() as i32 == 1 || c as i32 - open.chars().last().unwrap() as i32 == 2 { open.pop(); (open, is_faulty || false) } else { (open, true) } ) ).filter(|(_, is_faulty)| !is_faulty ).map(|(open, _)| if open.len() == 0 { 0 } else { open.chars().rev().fold(0_u64, |acc, c| acc * 5 + if c == '(' { 1 } else if c == '[' { 2 } else if c == '{' { 3 } else { 4 } ) } ).collect::<Vec<_>>();
    vals.sort();
    println!("{}", vals[vals.len() / 2]);
}