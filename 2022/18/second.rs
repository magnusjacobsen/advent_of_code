use std::io::BufRead;
use std::collections::HashSet;

fn get_neighbours(x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)>{
    let mut out = vec![];
    if x > 0  { out.push((x-1, y, z)); }
    if x < 22 { out.push((x+1, y, z)); }
    if y > 0  { out.push((x, y-1, z)); }
    if y < 22 { out.push((x, y+1, z)); }
    if z > 0  { out.push((x, y, z-1)); }
    if z < 22 { out.push((x, y, z+1)); }
    out
}

fn main() {
    let cubes = std::io::stdin().lock().lines().map(|line| {
        let cube = line.unwrap().split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        (cube[0] + 2, cube[1] + 2, cube[2] + 2)
    }).collect::<HashSet<(usize, usize, usize)>>();
    
    let mut unvisited = vec![vec![vec![true; 23]; 23]; 23];
    let mut stack = vec![(22, 22, 22)];
    let mut count = 0;
    while let Some((x, y, z)) = stack.pop() {
        if unvisited[z][y][x] {
            unvisited[z][y][x] = false;
            let neighbours = get_neighbours(x, y, z);
            for (nx, ny, nz) in neighbours {
                if cubes.contains(&(nx, ny, nz)) {
                    count += 1;
                } else {
                    stack.push((nx, ny, nz))
                }
            }
        }
    }

    println!("{}", count);
}