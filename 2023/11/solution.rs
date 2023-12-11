use std::io::BufRead;
use std::cmp::{max, min};

const EMPTY_DISTANCE: i64 = 2;//1_000_000;

fn is_row_empty(i: usize, grid: &Vec<Vec<char>>) -> bool {
    grid[i].iter().all(|x| x == &'.')
}

fn is_col_empty(j: usize, grid: &Vec<Vec<char>>) -> bool {
    grid.iter().all(|x| x[j] == '.')
}

fn get_distance((s1_i, s1_j): (usize, usize), (s2_i, s2_j): (usize, usize), space: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for i in min(s1_i, s2_i) + 1..max(s1_i, s2_i) + 1 {
        sum += space[i][s1_j];
    }
    for j in min(s1_j, s2_j) + 1..max(s1_j, s2_j) + 1  {
        sum += space[s1_i][j];
    }
    sum
} 

fn main() {
    let grid = std::io::stdin().lock().lines().map(|line| line.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut stars = vec![];
    let mut space = vec![vec![0; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let size = if is_row_empty(i, &grid) || is_col_empty(j, &grid) {  EMPTY_DISTANCE } else { 1 };
            if grid[i][j] == '#' { stars.push((i, j)); }
            space[i][j] = size;
        }
    }
    let num_stars = stars.len();
    let mut sum = 0;
    for i in 0..num_stars - 1 {
        for j in i + 1..num_stars {
            sum += get_distance(stars[i], stars[j], &space);
        }
    }
    println!("{}", sum);
}