#![feature(stdin_forwarders)]
fn main() {
    let input = std::io::stdin().lines().map(|x| x.unwrap().to_string()).collect::<Vec<_>>();
    let mut coords = vec![];
    let mut folds = vec![];    
    for elem in input {
        if elem.contains(",") {
            let coord = elem.split(",").map(|a| a.parse::<usize>().unwrap()).collect::<Vec<_>>();
            coords.push(coord);
        } else if elem.contains("fold") {
            let splt = elem.split(" ").collect::<Vec<_>>()[2].split("=").collect::<Vec<_>>();
            folds.push((splt[0] == "y", splt[1].parse::<usize>().unwrap()));
        }
    }
    let x_len = coords.iter().map(|a| a[0]).max().unwrap() + 1;
    let y_len = coords.iter().map(|a| a[1]).max().unwrap() + 1;
    let mut paper = vec![vec![0; x_len]; y_len];
    for coord in coords {
        paper[coord[1]][coord[0]] = 1;
    }
    
    for (y_axis, val) in [folds[0]] {
        let start_x = if y_axis { 0 } else { val };
        let start_y = if y_axis { val } else { 0 };
        for y in start_y..y_len {
            for x in start_x..x_len {
                if paper[y][x] == 1 {
                    paper[y][x] = 0;
                    if y_axis {
                        paper[val - (y - val)][x] = 1;
                    } else {
                        paper[y][val - (x - val)] = 1;
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            sum += paper[y][x];
        }
    }
    println!("sum: {}", sum);
}
