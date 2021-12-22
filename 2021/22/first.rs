#![feature(stdin_forwarders)]
use std::collections::HashSet;

// (x1, x2), (y1, y2), (z1, z2)
type Cube = ((i32, i32), (i32, i32), (i32, i32));

fn predicate((a,b): (i32, i32)) -> bool {
    a >= -50 && a <= 50 && b >= -50 && b <= 50
}

fn main() {
    let input = std::io::stdin().lines().map(|x| {
        let splt = x.unwrap().split(" ").map(|y| y.to_string()).collect::<Vec<_>>();
        let on = splt[0] == "on";
        let cube_vec = splt[1].split(",").map(|coord| {
            let vec = coord.split("=").collect::<Vec<_>>()[1].split("..").map(|z| z.parse::<i32>().unwrap()).collect::<Vec<_>>();
            if vec[0] > vec[1] { (vec[1], vec[0]) } else { (vec[0], vec[1]) }
        }).collect::<Vec<_>>();
        (on, (cube_vec[0], cube_vec[1], cube_vec[2]))
    }).collect::<Vec<(bool, Cube)>>();

    let mut state = HashSet::new();
    for step in 0..input.len() {
        let (on, cube) = &input[step];

        if !predicate(cube.0) && !predicate(cube.1) && !predicate(cube.2) {
            continue;
        }

        for x in cube.0.0..cube.0.1 + 1 {
            for y in cube.1.0..cube.1.1 + 1 {
                for z in cube.2.0..cube.2.1 + 1 {
                    if *on {
                        state.insert((x,y,z));
                    } else {
                        state.remove(&(x,y,z));
                    }
                }
            }
        }
    }
    println!("{}", state.len());
}