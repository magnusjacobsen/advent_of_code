use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let vectors = io::stdin().lock().lines().map(|x| x.unwrap().split(" -> ").map(|x| x.split(',').map(|y| i32::from_str(y).unwrap() ).collect::<Vec<_>>() ).collect::<Vec<_>>() ).collect::<Vec<_>>();

    let xmax = vectors.iter().map(|v| v.iter().map(|p| p[0]).max().unwrap()).max().unwrap() as usize;
    let ymax = vectors.iter().map(|v| v.iter().map(|p| p[1]).max().unwrap()).max().unwrap() as usize;
    let mut map = vec![vec![0; xmax + 1]; ymax + 1];

    vectors.iter()
        .for_each(|v| {
            if v[0][0] == v[1][0] { // x values are the same
                let (start, end) = if v[0][1] < v[1][1] {
                    (v[0][1], v[1][1] + 1)
                } else {
                    (v[1][1], v[0][1] + 1)
                };
                for i in start..end {
                    map[i as usize][v[0][0] as usize] += 1;
                }
            } else if v[0][1] == v[1][1] { // y values are the same
                let (start, end) = if v[0][0] < v[1][0] {
                    (v[0][0], v[1][0] + 1)
                } else {
                    (v[1][0], v[0][0] + 1)
                };
                for i in start..end {
                    map[v[0][1] as usize][i as usize] += 1;
                }
            // diagonal
            } else if (v[0][0] - v[1][0]).abs() == (v[0][1] - v[1][1]).abs() {
                let x_inc = if v[0][0] < v[1][0] { 1 } else { -1 };
                let y_inc = if v[0][1] < v[1][1] { 1 } else { -1 };
                let length = (v[0][0] - v[1][0]).abs() + 1;
                for i in 0..length {
                    map[(v[0][1] + y_inc * i) as usize][(v[0][0] + x_inc * i) as usize] += 1;
                }
            }
        });

    println!("overlaps: {}", (0..map.len()).fold(0, |accy, i| accy + (0..map[i].len()).fold(0, |accx, j| if map[i][j] > 1 { accx + 1 } else { accx })));
}