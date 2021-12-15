#![feature(stdin_forwarders)]
fn rec_flash(i: usize, j: usize, matrix: &mut Vec<Vec<u32>>) {
    matrix[i][j] += 1;
    if matrix[i][j] == 10 {
        let not_top = i > 0;
        let not_bottom = i < matrix.len() - 1;
        let not_left = j > 0;
        let not_right = j < matrix[0].len() - 1;
        if not_top { 
            rec_flash(i - 1, j, matrix);
            if not_left { rec_flash(i - 1, j - 1, matrix); }
            if not_right { rec_flash(i - 1, j + 1, matrix); }
        }
        if not_bottom {
            rec_flash(i + 1, j, matrix);
            if not_left { rec_flash(i + 1, j - 1, matrix); }
            if not_right { rec_flash(i + 1, j + 1, matrix); }
        }
        if not_left { rec_flash(i, j - 1, matrix); }
        if not_right { rec_flash(i, j + 1, matrix); }
    }
}

fn main() {
    let mut vals = std::io::stdin().lines().map(|x| x.unwrap().chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut num_flashes = 0;
    for _ in 0..100 {
        for i in 0..vals.len() {
            for j in 0..vals[0].len() {
                rec_flash(i, j, &mut vals);
            }
        }

        for i in 0..vals.len() {
            for j in 0..vals[0].len() {
                if vals[i][j] > 9 {
                    vals[i][j] = 0;
                    num_flashes += 1;
                }
            }
        }
    }
    println!("num flashes: {}", num_flashes);
}
