use std::io::{self, BufRead};

fn parse_line(x: &str) -> Vec<i32> {
    x.split_whitespace().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>()
}

fn main() {
    let lines = io::stdin().lock().lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    let mut potential_triangles = vec![];
    for three_lines in lines.chunks(3) {
        potential_triangles.push([three_lines[0][0], three_lines[1][0], three_lines[2][0]]);
        potential_triangles.push([three_lines[0][1], three_lines[1][1], three_lines[2][1]]);
        potential_triangles.push([three_lines[0][2], three_lines[1][2], three_lines[2][2]]);
    }
    
    let num_triangles = potential_triangles.iter_mut().filter(|x| x[0] > 0 && x[1] > 0 && x[2] > 0).fold(0, |acc, x| {
        x.sort();
        if x[0] + x[1] > x[2] { acc + 1 } else { acc }
    });

    println!("{}", num_triangles);
}