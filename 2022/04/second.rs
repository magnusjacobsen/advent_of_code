use std::io::BufRead;

fn main() {
    println!("{}", std::io::stdin().lock().lines().map(|line| line.unwrap().split(",").map(|elf| elf.split("-").map(|num| u32::from_str_radix(&num, 10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()).fold(0, |acc, pair| { if (pair[0][0] >= pair[1][0] && pair[0][0] <= pair[1][1] || pair[0][1] >= pair[1][0] && pair[0][1] <= pair[1][1]) || (pair[1][0] >= pair[0][0] && pair[1][0] <= pair[0][1] || pair[1][1] >= pair[0][0] && pair[1][1] <= pair[0][1]) { acc + 1} else { acc } }));
}