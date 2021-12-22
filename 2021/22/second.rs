#![feature(stdin_forwarders)]
type Coord = (i64, i64);
type Cube = (Coord, Coord, Coord);

fn intersects1(a: Coord, b: Coord) -> bool {
    a.1 >= b.0 && b.1 >= a.0
}

fn intersects3(a: &Cube, b: &Cube) -> bool {
    intersects1(a.0, b.0) && intersects1(a.1, b.1) && intersects1(a.2, b.2)
}

fn intersection(a: &Cube, b: &Cube) -> Cube {
    ((a.0.0.max(b.0.0), a.0.1.min(b.0.1)), (a.1.0.max(b.1.0), a.1.1.min(b.1.1)), (a.2.0.max(b.2.0), a.2.1.min(b.2.1)))
}

fn cube_size(a: &Cube) -> i64 {
    (a.0.1 - a.0.0) * (a.1.1 - a.1.0) * (a.2.1 - a.2.0)
}

fn rec_size(offs: Vec<Cube>) -> i64 {
    (0..offs.len()).fold(0_i64, |acc, i| acc + cube_size(&offs[i]) - rec_size((&offs[i + 1..]).iter().filter(|x| intersects3(x, &offs[i])).map(|x| intersection(x, &offs[i])).collect::<Vec<_>>()))
}

fn main() {
    let input = std::io::stdin().lines().map(|x| {
        let splt = x.unwrap().split(" ").map(|y| y.to_string()).collect::<Vec<_>>();
        let on = splt[0] == "on";
        let cube_vec = splt[1].split(",").map(|coord| {
            let vec = coord.split("=").collect::<Vec<_>>()[1].split("..").map(|z| z.parse::<i64>().unwrap()).collect::<Vec<_>>();
            if vec[0] > vec[1] { (vec[1], vec[0] + 1) } else { (vec[0], vec[1] + 1) }
        }).collect::<Vec<_>>();
        (on, (cube_vec[0], cube_vec[1], cube_vec[2]))
    }).collect::<Vec<(bool, Cube)>>();

    let mut offs: Vec<Vec<Cube>> = vec![];
    let mut on_cubes = vec![];
    for i in 0..input.len() {
        for j in 0..on_cubes.len() {
            if intersects3(&input[i].1, &on_cubes[j]) {
                offs[j].push(intersection(&input[i].1, &on_cubes[j]));
            }
        }
        if input[i].0 {
            on_cubes.push(input[i].1);
            offs.push(vec![])
        }
    }

    let count = (0..on_cubes.len()).fold(0_i64, |acc, i| acc + cube_size(&on_cubes[i]) - rec_size(offs[i].clone()));
    println!("{}", count);
}