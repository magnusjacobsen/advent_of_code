use std::io::BufRead;
use std::collections::HashMap;

fn get_minimum(start: &str, graph: &HashMap<&str, Vec<&str>>, instructions: &Vec<usize>) -> u64 {
    let mut current = start;
    let mut ins_idx = 0;
    let max_ins_idx = instructions.len() - 1;
    let mut steps = 0;
    loop {
        if &current[2..3] == "Z" {
            return steps;
        }
        current = graph[current][instructions[ins_idx]];
        steps += 1;
        ins_idx += 1;
        if ins_idx > max_ins_idx {
            ins_idx = 0;
        }
    }
}

/* from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs */
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

/* from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs */
fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 { return a; }
    gcd_of_two_numbers(b, a % b)
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect(); 
    let instructions: Vec<usize> = lines[0].chars().map(|c| if c == 'L' { 0 } else { 1 }).collect();
    let mut graph = HashMap::new();
    let mut starts = vec![];
    for i in 2..lines.len() {
        let key = &lines[i][0..3];// only ok because the input is ASCII
        let edges = vec![&lines[i][7..10], &lines[i][12..15]];
        if &key[2..3] == "A" { starts.push(key.to_string()); }
        graph.insert(key, edges);
    }
    let mut cycles = vec![];
    for start in starts {
        cycles.push(get_minimum(&start, &graph, &instructions));
    }
    println!("{}", lcm(&cycles[..]));
}