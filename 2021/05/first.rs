use std::io::{self, BufRead};
use std::str::FromStr;

// [] = index to all vectors
// [] = either point a or b (0 or 1)
// [] = either x or y value (0 or 1)
type Points = Vec<Vec<Vec<usize>>>;

// function to either find the min or max value
// the x-y index is found by converting a bool to usize (0 or 1)
#[inline(always)]
fn find_val(vectors: &Points, max: bool, y_val: bool) -> usize {
    if max {
        vectors.iter().map(|x| x.iter().map(|y| y[y_val as usize]).max().unwrap()).max().unwrap() as usize
    } else {
        vectors.iter().map(|x| x.iter().map(|y| y[y_val as usize]).min().unwrap()).min().unwrap() as usize
    }
}

fn main() {
    let vectors = io::stdin()
        .lock()
        .lines()
        .map(|x| { 
            x.unwrap()
                .split(" -> ")
                .map(|x| 
                    x.split(',')
                        .map(|y| 
                            usize::from_str(y)
                                .unwrap()
                        )
                        .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>()
        })
        //.filter(|z| z[0][0] == z[1][0] || z[0][1] == z[1][1])
        .collect::<Vec<_>>();

    let xmax = find_val(&vectors, true, false);
    let xmin = find_val(&vectors, false, false);
    let ymax = find_val(&vectors, true, true);
    let ymin = find_val(&vectors, false, true);

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
                    map[i][v[0][0]] += 1;
                }
            } else if v[0][1] == v[1][1] {
                let (start, end) = if v[0][0] < v[1][0] {
                    (v[0][0], v[1][0] + 1)
                } else {
                    (v[1][0], v[0][0] + 1)
                };
                for i in start..end {
                    map[v[0][1]][i] += 1;
                }
            }
        });
    
    let mut overlaps = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] > 1 {
                overlaps += 1;
            }
        }
    }
    println!("overlaps: {}", overlaps);
}