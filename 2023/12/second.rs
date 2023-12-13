use std::io::BufRead;
use std::collections::HashMap;

fn is_possible_gear(chars: &[u8], following: u8) -> bool {
    chars.iter().all(|x| x == &1 || x == &2) && following != 1
}

fn find_arrangements(chars: &[u8], numbers: &[usize], memory: &mut HashMap<(Vec<u8>, Vec<usize>), u64>) -> u64 {
    if numbers == [] {
        if chars.iter().all(|x| x != &1) {
            return 1;
        } else {
            return 0;
        }
    }
    let key = (chars.to_vec(), numbers.to_vec());
    if let Some(mem_val) = memory.get(&key) {
        return *mem_val;
    }
    let number = numbers[0];
    let char_len = chars.len();
    if char_len < number + 1 { // not space for last number
        return 0;
    } else if chars[0] == 0 {  // empty
        let num_arr = find_arrangements(&chars[1..], numbers, memory);
        memory.insert(key, num_arr);
        return num_arr;
    } else if chars[0] == 1 { // gear part
        if is_possible_gear(&chars[0..number], chars[number]) {
            let num_arr = find_arrangements(&chars[number + 1..], &numbers[1..], memory);
            memory.insert(key, num_arr);
            return num_arr;
        } else {
            return 0;
        };
    } else { // joker
        let a = if is_possible_gear(&chars[0..number], chars[number]) {
            find_arrangements(&chars[number + 1..], &numbers[1..], memory)
        } else {
            0
        };
        let b = find_arrangements(&chars[1..], numbers, memory);
        let num_arr = a + b;
        memory.insert(key, num_arr);
        return num_arr;
    }
}

fn extend_chars(input: Vec<u8>) -> Vec<u8> {
    [&input[..], &[2], &input[..], &[2], &input[..], &[2], &input[..], &[2], &input[..], &[0]].concat()
}

fn extend_nums(input: Vec<usize>) -> Vec<usize> {
    [&input[..], &input[..], &input[..], &input[..],&input[..]].concat()
}

// 0 = empty
// 1 = gear
// 2 = joker

fn main() {
    let lines: Vec<Vec<String>> = std::io::stdin().lock().lines().map(|line| line.unwrap().split(" ").map(|s| s.to_string()).collect()).collect();
    let mut sum = 0;
    for splitted in lines {
        let chars: Vec<u8> = extend_chars(splitted[0].chars().map(|x| if x == '.' { 0 } else if x == '#' { 1 } else { 2 }).collect());
        let numbers = extend_nums(splitted[1].split(",").map(|x| x.
            parse::<usize>().unwrap()).collect());
        let mut memory = HashMap::new();
        let num_arr = find_arrangements(&chars[..], &numbers[..], &mut memory);
        println!("num_arr: {}", num_arr);
        sum += num_arr;
    }
    println!("{}", sum);
}