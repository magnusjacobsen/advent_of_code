use std::io::BufRead;

fn main() {
    let scan = std::io::stdin().lock().lines().map(|line| {
        line.unwrap().split(" -> ").map(|pair| {
            let mut splt = pair.split(",");
            (splt.next().unwrap().parse().unwrap(), splt.next().unwrap().parse().unwrap())
        }).collect::<Vec<(usize, usize)>>()
    }).collect::<Vec<_>>();

    let (mut minx, mut maxx, mut miny, mut maxy) = (usize::MAX, 0, usize::MAX, 0);

    for i in 0..scan.len() {
        for j in 0..scan[i].len() {
            let (x, y) = scan[i][j];
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.max(y);
        }
    }

    let mut cave = vec![vec!['.'; maxx * 2]; maxy * 2];

    for i in 0..scan.len() {
        for j in 1..scan[i].len() {
            let (x1, y1) = scan[i][j - 1];
            let (x2, y2) = scan[i][j];
            let minx1 = x1.min(x2);
            let maxx1 = x1.max(x2);
            let miny1 = y1.min(y2);
            let maxy1 = y1.max(y2);
            for y in miny1..maxy1 + 1 {
                for x in minx1..maxx1 + 1 {
                    cave[y][x] = '▓';
                }
            }
        }
    }

    println!("{}", 2 + maxy);
    for x in 0..maxx * 2 {
        cave[2 + maxy][x] = '▓';
    }

    let mut units = 0;
    
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            if cave[y + 1][x] == '.' {
                y += 1;
            } else if cave[y + 1][x - 1] == '.' {
                y += 1;
                x -= 1;
            } else if cave[y + 1][x + 1] == '.' {
                y += 1;
                x += 1;
            } else {
                cave[y][x] = '░';
                units += 1;
                break;
            }
        }
        if (x, y) == (500, 0) {
            break;
        } 
    }

    /* for i in 0..maxy + 1 {
        println!("{}", cave[i][minx..maxx + 1].iter().collect::<String>());
    } */

    println!("{}", units);
}