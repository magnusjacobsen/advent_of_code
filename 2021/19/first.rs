#![feature(stdin_forwarders)]
use std::collections::{HashMap, HashSet};

fn subtract(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add(a: (i32, i32, i32), b: (i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn rotations(coord: (i32,i32,i32)) -> Vec<(i32,i32,i32)> {
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
    let xmult = if (i % 12) < 6 { 1 } else { -1 };
    let ymult = if (i % 24) < 12 { 1 } else { -1 };
    let zmult = if i < 24 { 1 } else { -1 };
    (x * xmult, y * ymult, z * zmult)
}

fn rec(current: usize, visited: &mut Vec<bool>, unique: &mut HashSet<(i32, i32, i32)>, edges: &Vec<Vec<(usize, (i32, i32, i32), usize)>>, current_rots: &Vec<usize>, current_dists: &Vec<(i32, i32, i32)>, scanners: &Vec<Vec<(i32, i32, i32)>>) {
    if !visited[current] {
        visited[current] = true;
        for beacon in &scanners[current] {
            let corrected = (0..current_dists.len()).fold(*beacon, |c, j| add(rotate(current_rots[j], c), current_dists[j]));
            unique.insert(corrected);
        }
        for edge in &edges[current] {
            let mut new_rots = vec![edge.2];
            let mut new_dists = vec![edge.1];
            new_rots.extend(current_rots);
            new_dists.extend(current_dists);
            rec(edge.0, visited, unique, edges, &new_rots, &new_dists, scanners);
        }
    }
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

    let mut edges = vec![vec![]; scanners.len()];

    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            if j == i {
                continue;
            }
            let mut map = HashMap::new();
            for ii in 0..scanners[i].len() {
                for jj in 0..scanners[j].len() {
                    let b_rotations = rotations(scanners[j][jj]);
                    for jjj in 0..b_rotations.len() {
                        *map.entry((subtract(scanners[i][ii], b_rotations[jjj]), jjj)).or_insert(0) += 1;
                    }
                }
            }
            let mut vec = map.iter().collect::<Vec<_>>();
            vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
            let cand = vec.iter().filter(|x| *x.1 > 11).collect::<Vec<_>>();
            if cand.len() > 0 {
                let ((rel_dist, rot), _) = cand[0];
                let edge = (j, rel_dist.clone(), *rot);
                edges[i].push(edge);
            }
        }
    }

    let mut visited = vec![false; scanners.len()];
    let mut unique = HashSet::new();

    rec(0, &mut visited, &mut unique, &edges, &vec![0], &vec![(0,0,0)], &scanners);
    println!("{}", unique.len());
}