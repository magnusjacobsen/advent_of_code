use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut sum = 0;
    io::stdin().lock().lines().for_each(|line| {
        let unwrapped = line.unwrap();
        let splitted = unwrapped.split("-").collect::<Vec<_>>();
        let first_part = splitted.iter().take(splitted.len() - 1).cloned().collect::<Vec<_>>().concat();
        let second_part = splitted[splitted.len() - 1].split("[").collect::<Vec<_>>();
        let sector_id = second_part[0].parse::<i32>().unwrap();
        let mut map = HashMap::new();
        for c in first_part.chars() {
            if c.is_alphabetic() {
                *map.entry(c).or_insert(0) += 1;
            }
        }
        let mut list = map.iter().collect::<Vec<_>>();
        list.sort_by(|a, b| if a.1 == b.1 {a.0.cmp(b.0)} else {b.1.cmp(a.1)});
        let letters: String = list.iter().take(5).map(|x| *x.0).collect();
        if second_part[1].starts_with(&letters) {
            sum += sector_id;
        }
    });
    println!("sum: {}", sum);
}