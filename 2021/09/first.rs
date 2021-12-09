#![feature(stdin_forwarders)]

fn pad(vec: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new = vec![vec![9; vec[0].len() + 2]; vec.len() + 2];
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            new[i + 1][j + 1] = vec[i][j];
        }
    }
    new
}

fn main() {
    let tmp = std::io::stdin().lines().map(|x| x.unwrap().chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let matrix = pad(tmp);

    let mut sum = 0;

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] < matrix[i - 1][j] && 
                matrix[i][j] < matrix[i][j - 1] &&
                matrix[i][j] < matrix[i + 1][j] && 
                matrix[i][j] < matrix[i][j + 1] {
                sum += matrix[i][j] + 1;
            }
        }
    }

    println!("{}", sum);
}