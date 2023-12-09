use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect();
    
    let instructions: Vec<usize> = lines[0].chars().map(|c| if c == 'L' { 0 } else { 1 }).collect();

    let mut mapping = HashMap::new();
    
    for i in 0..lines.len() - 2 {
        let key = lines[i + 2].split(" = ").collect::<Vec<_>>()[0];
        mapping.insert(key, i);
    }
    
    let mut graph = vec![];
    
    for i in 0..mapping.len() {
        let left_key = &lines[i + 2][7..10]; // only ok because the input is ASCII
        let right_key = &lines[i + 2][12..15];
        let dests = vec![mapping[left_key], mapping[right_key]];
        graph.push(dests);
    }

    let mut current = mapping["AAA"];
    let mut ins_idx = 0;
    let max_ins_idx = instructions.len() - 1;
    let mut steps = 0;
    let target = mapping["ZZZ"];

    while current != target {
        current = graph[current][instructions[ins_idx]];
        steps += 1;
        ins_idx += 1;
        if ins_idx > max_ins_idx {
            ins_idx = 0;
        }
    }
    println!("{}", steps);
}