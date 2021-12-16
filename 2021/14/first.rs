#![feature(stdin_forwarders)]
use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::io::stdin().lines().map(|x| x.unwrap().to_string()).collect::<Vec<_>>();
    let mut polymer = input[0].chars().map(|x| x as u32).collect::<Vec<_>>();
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for i in 2..input.len() {
        let splt = input[i].split(" -> ").collect::<Vec<_>>();
        let key = splt[0].chars().map(|x| x as u32).collect::<Vec<_>>();
        let val = splt[1].chars().next().unwrap() as u32;
        map.insert((key[0], key[1]), val);
        set.insert(val);
    }

    for _ in 0..10 {
        let mut new_polymer = vec![];
        for i in 0..polymer.len() - 1 {
            let new_element = map.get(&(polymer[i], polymer[i + 1])).unwrap();
            new_polymer.push(polymer[i]);
            new_polymer.push(*new_element);
        }
        new_polymer.push(polymer[polymer.len() - 1]);
        polymer = new_polymer;
    }
    let max = set.iter().map(|x| polymer.iter().filter(|y| *y == x).count()).max().unwrap();
    let min = set.iter().map(|x| polymer.iter().filter(|y| *y == x).count()).min().unwrap();
    println!("{}", max - min);
}