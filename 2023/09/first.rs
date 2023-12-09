use std::io::BufRead;

fn find_history(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|num| *num == 0) {
        return 0;
    }
    let last_index = sequence.len() - 1;
    let diff = (0..last_index).map(|i| sequence[i + 1] - sequence[i]).collect::<Vec<_>>();
    sequence[last_index] + find_history(&diff)
}

fn main() {
    let sequences: Vec<Vec<i32>> = std::io::stdin().lock().lines().map(|line| line.unwrap().split(' ').map(|num_str| num_str.parse::<i32>().unwrap()).collect()).collect();

    println!("{}", sequences.iter().fold(0, |acc, seq| acc + find_history(&seq)));
}