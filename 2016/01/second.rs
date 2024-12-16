use std::io::{self, BufRead};
use std::collections::{HashSet};

#[derive(Debug)]
enum Turn {
    L,
    R,
}

#[derive(Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn get_dir(turn: Turn, dir: Dir) -> Dir {
    match turn {
        Turn::L => match dir {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        },
        Turn::R => match dir { // R
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        },
    }
}

fn get_vector(dir: &Dir) -> (i32, i32) {
    match dir {
        &Dir::N => (0, 1),
        &Dir::E => (1, 0),
        &Dir::S => (0, -1),
        &Dir::W => (-1, 0),
    }
}

fn get_action(action_str: &str) -> (Turn, i32) {
    let turn = if &action_str[0..1] == "L" { Turn::L } else { Turn::R };
    let scalar = action_str[1..].parse::<i32>().unwrap();
    (turn, scalar)
}

fn main() {
    let mut visited = HashSet::new();
    let (_dir, (x, y)) = io::stdin()
        .lock()
        .lines()
        .next().unwrap().unwrap()
        .split(", ")
        .fold((Dir::N, (0, 0)), |(acc_dir, (acc_x, acc_y)), action_str| {
            let (turn, scalar) = get_action(action_str);
            let next_dir = get_dir(turn, acc_dir);
            let vec = get_vector(&next_dir);
            for i in 0..scalar {
                let key = (acc_x + vec.0 * i, acc_y + vec.1 * i);
                if visited.contains(&key) {
                    println!("{}", key.0.abs() + key.1.abs());
                }
                visited.insert(key);
            }
            (next_dir, (acc_x + vec.0 * scalar, acc_y + vec.1 * scalar))
        });
    println!("{}", x.abs() + y.abs());
}