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
        width: usize) -> (HashSet<((i32, i32), i32)>, bool) {
    let mut visited : HashSet<((i32, i32), i32)> = HashSet::new();
    let mut current = start_pos;
    let mut dir = start_dir;
    while current.0 >= 0 && current.0 < width as i32 && current.1 >= 0 && current.1 < height as i32 {
        if visited.contains(&(current, dir)) {
            return (visited, true);
        }
        visited.insert((current, dir));
        let next = next_position(current, dir);
        if obstacles.contains(&next) {
            dir = next_dir(dir);
        } else {
            current = next;
        }
    }
    (visited, false)
}

fn main() {
    let all_chars = io::stdin().lock().lines().map(|x| x.unwrap().chars().collect()).collect::<Vec<Vec<char>>>();
    let height = all_chars.len();
    let width = all_chars[0].len();
    let (obstacles, start_pos) = get_start(all_chars, height, width);
    let (og_visited, _) = traverse(obstacles.clone(), start_pos, 0, height, width);
    let possible_obs = og_visited.iter().map(|x| x.0).collect::<HashSet<(i32, i32)>>();

    let mut loop_count = 0;
    for new_obs in possible_obs {
        if new_obs == start_pos {
            continue;
        }
        let mut obs_clone = obstacles.clone();
        obs_clone.insert(new_obs.clone());
        let (_visited, is_loop) = traverse(obs_clone, start_pos, 0, height, width);
        if is_loop {
            loop_count += 1;
        }
    }
    println!("{}", loop_count);
}