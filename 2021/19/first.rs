#![feature(stdin_forwarders)]
use std::collections::{HashMap, HashSet};

fn subtract(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn all_rotations(coord: (i32,i32,i32)) -> Vec<(i32,i32,i32)> {
    (0..48).map(|i| rotate(i, coord)).collect()
}

fn rotate(i: usize, vec: (i32,i32,i32)) -> (i32,i32,i32) {
    let (x, y, z) = match i % 6 {
        0 => (vec.0, vec.1, vec.2),
        1 => (vec.0, vec.2, vec.1),
        2 => (vec.1, vec.0, vec.2),
        3 => (vec.1, vec.2, vec.0),
        4 => (vec.2, vec.1, vec.0),
        _ => (vec.2, vec.0, vec.1),
    };
    (
        x * if (i % 12) < 6 { 1 } else { -1 },
        y * if (i % 24) < 12 { 1 } else { -1 },
        z * if i < 24 { 1 } else { -1 }
    )
    
}

fn main() {
    let input = std::io::stdin().lines().map(|x| x.unwrap().to_string()).collect::<Vec<_>>();
    let mut index = 0;
    let mut scanners = vec![];
    while index < input.len() {
        let mut scanner = vec![];
        index += 1;
        while index < input.len() && input[index] != "" {
            let beacon = input[index].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            scanner.push((beacon[0], beacon[1], beacon[2]));
            index += 1;
        }
        scanners.push(scanner);
        index += 1;
    }
    let mut unique = scanners[0].iter().map(|x| *x).collect::<HashSet<(i32, i32 ,i32)>>();
    let mut distances = vec![(0, 0, 0); scanners.len()];
    let mut from = (0..scanners.len()).collect::<Vec<_>>();
    let mut rotations = vec![0; scanners.len()];
    let mut queue = vec![0];
    let mut visited = vec![false; scanners.len()];

    while let Some(current) = queue.pop() {
        if visited[current] {
            continue;
        }
        visited[current] = true;
        for other in 0..scanners.len() {
            if other == current || visited[other] || from[other] != other {
                continue;
            }
            let mut map = HashMap::new();
            'outer: for i in 0..scanners[current].len() {
                for j in 0..scanners[other].len() {
                    let other_rotations = all_rotations(scanners[other][j]);
                    for rotj in 0..other_rotations.len() {
                        let val = map.entry((subtract(scanners[current][i], other_rotations[rotj]), rotj)).or_insert(0);
                        *val += 1;
                        if *val > 11 {
                            break 'outer;
                        }
                    }
                }
            }
            if let Some(((rel_dist, rot), _)) = map.iter().filter(|x| *x.1 > 11).next() {
                from[other] = current;
                distances[other] = rel_dist.clone();
                rotations[other] = *rot;
                queue.push(other);

                let mut other_from = other;
                let mut other_dists = vec![(0, 0, 0)];
                let mut other_rots = vec![0];
                while other_from != 0 {
                    other_dists.push(distances[other_from]);
                    other_rots.push(rotations[other_from]);
                    other_from = from[other_from];
                }

                for beacon in &scanners[other] {
                    let corrected = (0..other_dists.len()).fold(*beacon, |acc, x| add(rotate(other_rots[x], acc), other_dists[x]));
                    unique.insert(corrected);
                }
            }
        }
    }
    println!("{}", unique.len());
}