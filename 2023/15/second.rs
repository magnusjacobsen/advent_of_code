use std::io::BufRead;

fn hash(chars: &Vec<char>) -> usize {
    chars.iter().fold(0, |current, c| ((current + *c as usize) * 17) % 256)
}

fn get_equal(chars: &Vec<char>) -> (usize, Vec<char>, u64) {
    let sign_index = chars.iter().position(|r| r == &'=').unwrap();
    let number = chars[sign_index + 1..].iter().rev().enumerate().fold(0, |sum, (i, x)| sum + (x.to_digit(10).unwrap() as u64 * 10_u64.pow(i as u32)));
    let key = chars[0..sign_index].to_vec();
    let hash = hash(&key);
    (hash, key, number)
}

fn get_dash(chars: &Vec<char>) -> (usize, Vec<char>) {
    let key = chars[0..chars.len() - 1].to_vec();
    let hash = hash(&key);
    (hash, key)
}

fn get_new_equal_box(chars: Vec<char>, focal: u64, hash_box: &Vec<(Vec<char>, u64)>) -> Vec<(Vec<char>, u64)> {
    let mut new_hash_box = hash_box.clone();
    for i in 0..new_hash_box.len() {
        if new_hash_box[i].0 == chars {
            new_hash_box[i] = (chars, focal);
            return new_hash_box;
        }
    }
    new_hash_box.push((chars, focal));
    new_hash_box
}

fn get_new_dash_box(chars: Vec<char>, hash_box: &Vec<(Vec<char>, u64)>) -> Vec<(Vec<char>, u64)> {
    hash_box.iter().filter(|x| x.0 != chars).cloned().collect::<Vec<_>>()
}

fn do_op(chars: &Vec<char>, map: &mut Vec<Vec<(Vec<char>, u64)>>) {
    let len = chars.len();
    if chars[len - 1] == '-' {
        let (hash, key) = get_dash(chars);
        map[hash] = get_new_dash_box(key, &map[hash]);
    } else {
        let (hash, key, focal) = get_equal(chars);
        map[hash] = get_new_equal_box(key, focal, &map[hash]);
    }
}

fn get_total_focusing_power(map: &Vec<Vec<(Vec<char>, u64)>>) -> u64 {
    let mut sum = 0;
    for (i, boxes) in map.iter().enumerate() {
        for (j, (_key, focal)) in boxes.iter().enumerate() {
            sum += ((i + 1) * (j + 1)) as u64 * focal;
        }
    }
    sum
}

fn main() {
    let input = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>()[0].split(",").map(|elem| elem.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut map: Vec<Vec<(Vec<char>, u64)>> = vec![vec![]; 256];
    for chars in input {
        do_op(&chars, &mut map);
    }
    println!("{}", get_total_focusing_power(&map));
}