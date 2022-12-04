use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

fn parse_input() -> (Vec<(bool, usize)>, Vec<String>, Vec<(u64, u64)>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter(|x| x.as_ref().unwrap() != "");
    let mut action_is_assignment = vec![];
    let mut masks = vec![];
    let mut assignments = vec![];
    while let Some(v) = lines.next() {
        let tmp = v.unwrap().to_string();
        if tmp.contains("mask") {
            let mask_vec: Vec<&str> = tmp.split(" = ").collect();
            let mask_str: String = mask_vec[1].to_string();
            masks.push(mask_str);
            action_is_assignment.push((false, masks.len() - 1));
        } else {
            let mem_vec: Vec<&str> = tmp.split(" = ").collect();
            let address_str: String = mem_vec[0].chars()
                .filter(|x| x != &'m' && x != &'e' && x != &'[' && x != &']')
                .collect();
            let address: u64 = u64::from_str(&address_str).unwrap();
            let value: u64 = u64::from_str(mem_vec[1]).unwrap();
            assignments.push((address, value));
            action_is_assignment.push((true, assignments.len() - 1));
        }
    }
    (action_is_assignment, masks, assignments)
}

fn apply_mask(mask: &str, index: i32, address: u64, value: u64, memory: &mut HashMap<u64, u64>) {
    if mask.is_empty() {
        memory.insert(address, value);
    } else {
        match &mask.chars().next().unwrap() {
            '1' => {
                let new_address = address | 2_u64.pow(index as u32);
                apply_mask(&mask[1..mask.len()], index - 1, new_address, value, memory);
            }
            '0' => {
                apply_mask(&mask[1..mask.len()], index - 1, address, value, memory);
            }
            _   => {
                let a = address | 2_u64.pow(index as u32);
                let b = address & (std::u64::MAX ^ 2_u64.pow(index as u32));
                apply_mask(&mask[1..mask.len()], index - 1, a, value, memory);
                apply_mask(&mask[1..mask.len()], index - 1, b, value, memory);
            }
        }
    }
}

fn calculate_memory(action_is_assignment: Vec<(bool, usize)>, masks: Vec<String>, assignments: Vec<(u64, u64)>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask_id = 0;
    for (is_assignment, i) in action_is_assignment {
        if is_assignment {
            let (address, value) = assignments[i];
            apply_mask(&masks[mask_id], 35, address, value, &mut memory);
        } else {
            mask_id = i;
        }
    }    

    let mut sum = 0;
    for (_, value) in &memory {
        sum += value;
    }
    sum
}

fn main() {
    let (is_assignment, masks, assignments) = parse_input();
    let result = calculate_memory(is_assignment, masks, assignments);
    println!("{}", result);
}