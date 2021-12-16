#![feature(stdin_forwarders)]
use std::collections::HashMap;

fn merge(a: [u64; 10], b: [u64; 10]) -> [u64; 10] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3], a[4] + b[4], a[5] + b[5], a[6] + b[6], a[7] + b[7], a[8] + b[8], a[9] + b[9]]
}

fn rec(a: usize, b: usize, remaining: usize, map: &HashMap<(usize, usize), usize>, mem: &mut HashMap<(usize, usize, usize), [u64;10]>, reverse: &HashMap<usize, char>) -> [u64; 10] {
    if remaining == 0 {
        let mut v = [0; 10];
        v[a] = 1;
        v
    } else {
        if let Some(v) = mem.get(&(a, b, remaining)) {
            v.clone()
        } else {
            let new = map.get(&(a, b)).unwrap();
            let v1 = rec(a, *new, remaining - 1, map, mem, reverse);
            let v2 = rec(*new, b, remaining - 1, map, mem, reverse);
            let v = merge(v1, v2);
            mem.insert((a, b, remaining), v.clone());
            v
        }
    }
}

fn find_index(c: char, current: usize, to_index: &mut HashMap<char, usize>, reverse: &mut HashMap<usize, char>) -> (usize, usize) {
    let mut next = current;
    let index = match to_index.get(&c) {
        Some(v) => *v,
        None => {
            to_index.insert(c, current);
            reverse.insert(current, c);
            next += 1;
            current
        },
    };
    (index, next)
}

/*
    Today's strategy: dynamic programmin
*/
fn main() {
    let input = std::io::stdin().lines().map(|x| x.unwrap().to_string()).collect::<Vec<_>>();
    let mut map = HashMap::new();
    let mut next_index = 0; // we are finding index values 0.. as letters occur
    let mut to_index = HashMap::new();
    let mut reverse = HashMap::new(); // reverse lookup of index to letter
    let steps = 40;

    // first iterate all the new polymer rules
    for i in 2..input.len() {
        let splt = input[i].split(" -> ").collect::<Vec<_>>();
        let (val, next) = find_index(splt[1].chars().next().unwrap(), next_index, &mut to_index, &mut reverse);
        next_index = next;
        let key = splt[0].chars().map(|k| {
            let (index, next) = find_index(k, next_index, &mut to_index, &mut reverse);
            next_index = next;
            index
        }).collect::<Vec<_>>();
        map.insert((key[0], key[1]), val);
    }
    // iterate the starting polymer string
    let polymer = input[0].chars().map(|x| *to_index.get(&x).unwrap()).collect::<Vec<_>>();

    let mut mem = HashMap::new(); // memory for dynamic programming
    // do recursive stuff
    let mut counts = (0..polymer.len() - 1)
        .fold([0; 10], |acc, i|
            merge(acc, rec(polymer[i], polymer[i + 1], steps, &map, &mut mem, &reverse)));
    counts[polymer[polymer.len() - 1]] += 1;

    let max = counts.iter().max().unwrap();
    let min = counts.iter().filter(|x| **x > 0).min().unwrap();

    println!("max: {}\nmin: {}\nmax - min: {}", max, min, max - min);
}