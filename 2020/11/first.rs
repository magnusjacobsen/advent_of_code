use std::io::{self, BufRead};

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Empty,
    Floor,
    Occupied,
}

fn parse_input() -> Vec<Vec<State>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut output: Vec<Vec<State>> = vec![];
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" { continue; }
        let mut states = vec![];
        for c in tmp.chars() {
            let s = match c {
                '#' => State::Occupied,
                '.' => State::Floor,
                'L' => State::Empty,
                _ => State::Floor,
            };
            states.push(s);
        }
        output.push(states);
    }
    output
}

fn is_occupied(i: isize, j: isize, states: &Vec<Vec<State>>) -> bool {
    let height = states.len() as isize;
    let width = states[0].len() as isize;
    i >= 0 && j >= 0 && i < height && j < width && states[i as usize][j as usize] == State::Occupied
}

// count occupied seats around position i, j
fn count(i: isize, j: isize, states: &Vec<Vec<State>>) -> i32 {
    let pos = vec![(i-1, j-1), (i-1,j), (i-1,j+1), (i,j-1), (i, j+1), (i+1,j-1), (i+1,j), (i+1,j+1)];
    let mut count = 0;
    for p in pos {
        count += if is_occupied(p.0, p.1, states) { 1 } else { 0 };
    }
    count
}

fn find_equilibrium(states: Vec<Vec<State>>) -> usize {
    let mut changes = 0;
    let mut copy = states.clone();
    for i in 0..states.len() {
        for j in 0..states[0].len() {
            match states[i][j] {
                State::Occupied if count(i as isize, j as isize, &states) > 3 => {
                    changes += 1;
                    copy[i][j] = State::Empty;
                },
                State::Empty if count(i as isize, j as isize, &states) == 0 => {
                    changes += 1;
                    copy[i][j] = State::Occupied;
                }
                _ => {},
            }
        }
    }

    if changes > 0 {
        return find_equilibrium(copy);
    } else {
        return copy.iter()
            .map(|x| x.iter()
                      .filter(|y| y == &&State::Occupied)
                      .count())
            .sum();
    }
}

fn main() {
    let states = parse_input();
    let result = find_equilibrium(states);
    println!("{}", result);
}