use std::io::{self, BufRead};

const FORWARDS: [char; 4] = ['X', 'M', 'A', 'S'];
const BACKWARDS : [char; 4] = ['S', 'A', 'M', 'X'];

fn search(all_chars: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    if j < all_chars[0].len() - 3 { // looking right
        let slice_right = [all_chars[i][j], all_chars[i][j+1], all_chars[i][j+2], all_chars[i][j+3]];
        if slice_right == FORWARDS { count += 1; }
        else if slice_right == BACKWARDS { count += 1; }
    }
    if i < all_chars.len() - 3 { // looking down
        let slice_down = [all_chars[i][j], all_chars[i+1][j], all_chars[i+2][j], all_chars[i+3][j]];
        if slice_down == FORWARDS { count += 1; }
        else if slice_down == BACKWARDS { count += 1; }
    }
    if i < all_chars.len() - 3 && j < all_chars[0].len() - 3 { // diagonal right down
        let slice_diag_right = [all_chars[i][j], all_chars[i+1][j+1], all_chars[i+2][j+2], all_chars[i+3][j+3]];
        let slice_diag_left = [all_chars[i][j+3], all_chars[i+1][j+2], all_chars[i+2][j+1], all_chars[i+3][j]];
        if slice_diag_right == FORWARDS { count += 1; }
        else if slice_diag_right == BACKWARDS { count += 1; }
        if slice_diag_left == FORWARDS { count += 1; }
        else if slice_diag_left == BACKWARDS { count += 1; }
    }
    count
}

fn main() {
    let all_chars = io::stdin().lock().lines().map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let height = all_chars.len();
    let width = all_chars[0].len();
    let all_count = (0..height).fold(0, |acc, i| acc + (0..width).fold(0, |acc2, j| acc2 + search(&all_chars, i, j)));
    println!("{}", all_count);
}