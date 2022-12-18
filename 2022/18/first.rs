use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let cubes = std::io::stdin().lock().lines().map(|line| {
        let cube = line.unwrap().split(",").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        (cube[0], cube[1], cube[2])
    }).collect::<HashSet<(i32, i32, i32)>>();
    let mut count = 0;
    for (x,y,z) in &cubes {
        if !cubes.contains(&(x-1, *y, *z)) { count += 1}
        if !cubes.contains(&(x+1, *y, *z)) { count += 1}
        if !cubes.contains(&(*x, y-1, *z)) { count += 1}
        if !cubes.contains(&(*x, y+1, *z)) { count += 1}
        if !cubes.contains(&(*x, *y, z-1)) { count += 1}
        if !cubes.contains(&(*x, *y, z+1)) { count += 1}
    }

    println!("{}", count);
}