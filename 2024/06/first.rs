use std::io::{self, BufRead};
use std::collections::HashSet;

fn next_dir(dir: i32) -> i32 {
    (dir + 1) % 4
}

fn next_position(current: (i32, i32), dir: i32) -> (i32, i32) {
    match dir {
        0 => (current.0, current.1 - 1),
        1 => (current.0 + 1, current.1),
        2 => (current.0, current.1 + 1),
        3 => (current.0 - 1, current.1),
        _ => panic!("")
    }
}

fn get_start(all_chars: Vec<Vec<char>>, height: usize, width: usize) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let mut current = (-1, -1);
    let mut obstacles : HashSet<(i32, i32)> = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            let pos = (x as i32, y as i32);
            if all_chars[y][x] == '#' {
                obstacles.insert(pos);
            } else if all_chars[y][x] == '^' {
                current = pos;
            }
        }
    }
    (obstacles, current)
}

fn traverse(obstacles: HashSet<(i32, i32)>, start_pos: (i32, i32), start_dir: i32, height: usize, 
        width: usize) -> HashSet<(i32, i32)> {
    let mut visited : HashSet<(i32, i32)> = HashSet::new();
    let mut current = start_pos;
    let mut dir = start_dir;
    while current.0 >= 0 && current.0 < width as i32 && current.1 >= 0 && current.1 < height as i32 {
        visited.insert(current);
        let next = next_position(current, dir);
        if obstacles.contains(&next) {
            dir = next_dir(dir);
        } else {
            current = next;
        }
    }
    visited
}

fn main() {
    let all_chars = io::stdin().lock().lines().map(|x| x.unwrap().chars().collect()).collect::<Vec<Vec<char>>>();
    let height = all_chars.len();
    let width = all_chars[0].len();
    let (obstacles, current) = get_start(all_chars, height, width);
    let visited = traverse(obstacles, current, 0, height, width);
    println!("{}", visited.len());
}