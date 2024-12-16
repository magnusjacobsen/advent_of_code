use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

fn create_antinodes(pos: (i32, i32), diff: (i32, i32), mut antinodes: Vec<(i32, i32)>, width: i32, height: i32) 
    -> Vec<(i32, i32)> {
    let antinode = (pos.0 + diff.0, pos.1 + diff.1);
    if is_valid(antinode, width, height) {
        antinodes.push(antinode);
        return create_antinodes(antinode, diff, antinodes, width, height);
    }
    antinodes
}

fn is_valid(antinode: (i32, i32), width: i32, height: i32) -> bool {
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < width && antinode.1 < height
}

fn main() {
    let all_chars = io::stdin().lock().lines().map(|x| x.unwrap().chars().collect()).collect::<Vec<Vec<char>>>();
    let height = all_chars.len() as i32;
    let width = all_chars[0].len() as i32;
    let mut freqs: HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    let mut uniq_antinodes : HashSet<(i32, i32)> = HashSet::new();
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
                let mut antinodes = vec![];
                let diff_a = (vec[i - 1].0 - vec[j].0, vec[i - 1].1 - vec[j].1);
                let diff_b = (vec[j].0 - vec[i - 1].0, vec[j].1 - vec[i - 1].1);
                antinodes = create_antinodes(vec[i - 1], diff_a, antinodes, width, height);
                antinodes = create_antinodes(vec[j], diff_b, antinodes, width, height);
                for antinode in antinodes {
                    uniq_antinodes.insert(antinode);
                }
                uniq_antinodes.insert(vec[i - 1]);
                uniq_antinodes.insert(vec[j]);
            }
        }
    }

    println!("{}", uniq_antinodes.len());
}