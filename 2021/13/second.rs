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
  
    for (y_axis, val) in folds {
        println!("\nNew fold");
        let paper_x_len = if y_axis { paper[0].len() } else { val };
        let paper_y_len = if y_axis { val } else { paper.len() };
        let mut new_paper = vec![vec![0; paper_x_len]; paper_y_len];
        for y in 0..paper.len() {
            for x in 0..paper[0].len() {
                if paper[y][x] == 1 {
                    if y_axis && y > val {
                        new_paper[val - (y - val)][x] = 1;
                    } else if !y_axis && x > val {
                        new_paper[y][val - (x - val)] = 1;
                    } else {
                        new_paper[y][x] = 1;
                    }
                }
            }
        }
        paper = new_paper;
    }

    let mut sum = 0;
    for i in 0..paper.len() {
        for j in 0..paper[0].len() {
            let c = if paper[i][j] == 1 { '#' } else { '.' };
            print!("{}", c);
        }
        println!("");
    }
}
