#![feature(stdin_forwarders)]
fn pad(vec: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new = vec![vec![0; vec[0].len() + 2]; vec.len() + 2];
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            new[i + 1][j + 1] = vec[i][j];
        }
    }
    new
}

fn paint_rec(val: u32, i: usize, j: usize, matrix: &mut Vec<Vec<u32>>) {
    if matrix[i][j] == 1 {
        matrix[i][j] = val;
        paint_rec(val, i + 1, j, matrix);
        paint_rec(val, i - 1, j, matrix);
        paint_rec(val, i, j + 1, matrix);
        paint_rec(val, i, j - 1, matrix);
    }
}

fn main() {
    let tmp = std::io::stdin().lines().map(|x| x.unwrap().chars().map(|y| if y.to_digit(10).unwrap() == 9 { 0 } else { 1 }).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut matrix = pad(tmp);

    let mut next_cluster_id = 2;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] == 1 {
                paint_rec(next_cluster_id, i, j, &mut matrix);
                next_cluster_id += 1;
            }
        }
    }

    let mut counts = (2..next_cluster_id).map(|x| matrix.iter().flatten().filter(|y| ***y == x).count()).collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));
    
    println!("{}", counts[0] * counts[1] * counts[2]);
}