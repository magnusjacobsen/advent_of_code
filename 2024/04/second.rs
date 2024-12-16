use std::io::{self, BufRead};

const FORWARDS: [char; 3] = ['M', 'A', 'S'];
const BACKWARDS : [char; 3] = ['S', 'A', 'M'];

fn search(all_chars: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    if i < all_chars.len() - 2 && j < all_chars[0].len() - 2 {
        let slice_diag_right = [all_chars[i][j], all_chars[i+1][j+1], all_chars[i+2][j+2]];
        let slice_diag_left = [all_chars[i][j+2], all_chars[i+1][j+1], all_chars[i+2][j]];
        if (slice_diag_right == FORWARDS || slice_diag_right == BACKWARDS) &&
            (slice_diag_left == FORWARDS || slice_diag_left == BACKWARDS) { 
                count += 1; 
        }
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