use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::HashMap;

fn parse_input() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().filter(|x| x.as_ref().unwrap() != "");
    let mut mask_ones: u64 = 0;
    let mut mask_zeros: u64 = std::u64::MAX;
    let mut mask_jokers: u64 = std::
    let mut memory: HashMap<u32, u64> = HashMap::new();
    while let Some(v) = lines.next() {
        let tmp = v.unwrap().to_string();
        if tmp.contains("mask") {
            let mask_vec: Vec<&str> = tmp.split(" = ").collect();
            let mask_str = mask_vec[1];

            mask_ones = 0;
            mask_zeros = std::u64::MAX;
            for (i, c) in mask_str.chars().rev().enumerate() {
                match c {
                    '1' => mask_ones |= 2_u64.pow(i as u32),
                    '0' => mask_zeros &= std::u64::MAX ^ 2_u64.pow(i as u32),
                    _ => {},
                }
            }
        } else {
            let mem_vec: Vec<&str> = tmp.split(" = ").collect();
            let address_str: String = mem_vec[0].chars()
                .filter(|x| x != &'m' && x != &'e' && x != &'[' && x != &']')
                .collect();
            let address: u32 = u32::from_str(&address_str).unwrap();
            let mut value: u64 = u64::from_str(mem_vec[1]).unwrap();
            value |= mask_ones;
            value &= mask_zeros;
            memory.insert(address, value);
        }
    }
    let mut sum = 0;
    for (_, value) in &memory {
        sum += value;
    }
    println!("{}", sum);
}

fn main() {
    parse_input();
}