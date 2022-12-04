use std::io::{self, BufRead};

fn parse_input() -> (Vec<Vec<usize>>, Vec<(usize, Vec<usize>)>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut idx: Vec<Vec<usize>> = vec![];
    let mut flat_states = vec![];    
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" { continue; }
        let row_index = idx.len();
        idx.push(vec![]);
        for c in tmp.chars() {
            if c == '#' { // occupied == 1 
                idx[row_index].push(flat_states.len());
                flat_states.push((1, vec![]));
            } else if c == 'L' { // empty == 0
                idx[row_index].push(flat_states.len());
                flat_states.push((0, vec![]));
            } else { // floor == 2
                idx[row_index].push(flat_states.len());
                flat_states.push((2, vec![]));
            }
        }
    }
    (idx, flat_states)
}

fn add_connections(idx: Vec<Vec<usize>>, mut flat: Vec<(usize, Vec<usize>)>) -> Vec<(usize, Vec<usize>)> {
    let height = idx.len();
    let width = idx[0].len();

    for i in 0..height {
        'a: for j in 0..width {
            let current = idx[i][j];
            if flat[current].0 == 2 { 
                continue 'a; 
            }
            'right: for k in 1..width - j {
                let friend = idx[i][j + k];
                if flat[friend].0 == 2 {
                    continue 'right;
                } 
                flat[current].1.push(friend);
                flat[friend].1.push(current);
                break 'right;
            }
            'down: for k in 1..height - i {
                let friend = idx[i + k][j];
                if flat[friend].0 == 2 {
                    continue 'down;
                } 
                flat[current].1.push(friend);
                flat[friend].1.push(current);
                break 'down;
            }
            'bottomright: for k in 1..std::cmp::min(width - j, height - i) {
                let friend = idx[i + k][j + k];
                if flat[friend].0 == 2 {
                    continue 'bottomright;
                }                 
                flat[current].1.push(friend);
                flat[friend].1.push(current);
                break 'bottomright;
            }
            'bottomleft: for k in 1..std::cmp::min(j + 1, height - i) {
                let friend = idx[i + k][j - k];
                if flat[friend].0 == 2 {
                    continue 'bottomleft;
                } 
                flat[current].1.push(friend as usize);
                flat[friend as usize].1.push(current);
                break 'bottomleft;
            }
        }
    }
    flat
}


fn find_equilibrium(states: Vec<(usize, Vec<usize>)>) -> usize {
    let mut changes = 0;
    let mut copy = states.clone();
    for i in 0..states.len() {
        let (state, connections) = &states[i];
        let mut count = 0;
        for c in connections {
            count += states[*c].0;
        }
        if *state == 0 && count == 0 {
            copy[i].0 = 1;
            changes += 1;
        } else if *state == 1 && count > 4 {
            copy[i].0 = 0;
            changes += 1;
        }
    }

    if changes > 0 {
        return find_equilibrium(copy);
    } else {
        return copy.iter()
                   .filter(|y| y.0 == 1)
                   .count()
    }
}

fn main() {
    let (idx, mut flat_states) = parse_input();
    flat_states = add_connections(idx, flat_states);
    let result = find_equilibrium(flat_states);
    println!("{}", result);
}