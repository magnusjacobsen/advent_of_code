use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn create_antinodes(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    ((a.0 * 2 - b.0, a.1 * 2 - b.1), (b.0 * 2 - a.0, b.1 * 2 - a.1))
}

fn is_valid(antinode: (i32, i32), width: i32, height: i32) -> bool {
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < width && antinode.1 < height
}

fn main() {
    let all_chars = io::stdin().lock().lines().map(|x| x.unwrap().chars().collect()).collect::<Vec<Vec<char>>>();
    let height = all_chars.len() as i32;
    let width = all_chars[0].len() as i32;
    let mut freqs: HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    let mut antinodes : HashSet<(i32, i32)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let c = all_chars[y as usize][x as usize];
            if c != '.' {
                (*freqs.entry(c).or_insert(vec![])).push((x, y));
            }
        }
    }
    for vec in freqs.values() {
        for i in 1..vec.len() {
            for j in i..vec.len() {
                let (anti1, anti2) = create_antinodes(vec[i - 1], vec[j]);
                if is_valid(anti1, width, height) {
                    antinodes.insert(anti1);
                }
                if is_valid(anti2, width, height) {
                    antinodes.insert(anti2);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}