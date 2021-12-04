use std::io::{self, BufRead};
use std::str::FromStr;

/*
    my approach is to have 2x2d vectors, 
    both in row and column form (row transposed), 
    for each board.

    Each 2d vector is 5x5
    and when a row consists of true values, 
    we have a winner!
*/
type Board = Vec<Vec<bool>>;

fn empty_board() -> Board {
    vec![vec![false; 5]; 5]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let numbers = lines.next().unwrap().unwrap().split(',').map(|x| usize::from_str(&x).unwrap()).collect::<Vec<_>>();
    lines.next();
    let mut boards: Vec<(Board, Board)> = vec![];
    let mut num_boards: Vec<Vec<Vec<usize>>> = vec![];

    let line_chunks: Vec<Vec<String>> = lines.collect::<Vec<_>>()
        .chunks(6)
        .into_iter()
        .map(|x| x.iter().map(|y| y.as_ref().unwrap().clone()).collect())
        .collect();
    for board in line_chunks {
        let mut num_board = vec![];
        for i in 0..5 {
            num_board.push((&board[i]).split_whitespace().map(|x| usize::from_str(x).unwrap()).collect::<Vec<usize>>());
        }
        num_boards.push(num_board);
        boards.push((empty_board(), empty_board()));
    }

    let mut inactive = vec![false; boards.len()];
    let mut num_inactive = 0;

    'drawing: for num in numbers {
        'boards: for idx in 0..boards.len() {
            if inactive[idx] {
                continue 'boards;
            }
            for i in 0..5 {
                for j in 0..5 {
                    if num_boards[idx][i][j] == num {
                        boards[idx].0[i][j] = true;
                        boards[idx].1[j][i] = true;
                        if boards[idx].0[i].iter().all(|x| *x) || boards[idx].1[j].iter().all(|x| *x) {
                            num_inactive += 1;
                            inactive[idx] = true;
                            if num_inactive < boards.len() {
                                continue 'boards;
                            }
                            let mut sum = 0;
                            for k in 0..5 {
                                for l in 0..5 {
                                    if boards[idx].0[k][l] == false {
                                        sum += num_boards[idx][k][l];
                                    }
                                }
                            }
                            println!("sum:        {}", sum);
                            println!("final num:  {}", num);
                            println!("multiplied: {}", sum * num);
                            break 'drawing;
                        }
                        continue 'boards;
                    }
                }
            }        
        }
    }
}
