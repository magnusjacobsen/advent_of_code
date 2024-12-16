use std::io::{self, BufRead};

const U_BORDERS : [i32; 5]  = [5, 2, 1, 4, 9];
const U_TWOS : [i32; 2]     = [3, 13];
const R_BORDERS : [i32; 5]  = [1, 4, 9, 12, 13];
const D_BORDERS : [i32; 5]  = [9, 12, 13, 10, 5];
const D_TWOS : [i32; 2]     = [1, 11];
const L_BORDERS : [i32; 5]  = [13, 10, 5, 2, 1];

fn do_action(p: i32, action: &char) -> i32 {
    match action {
        'U' => if U_BORDERS.contains(&p) { p } else if U_TWOS.contains(&p) { p - 2 } else { p - 4 },
        'R' => if R_BORDERS.contains(&p) { p } else { p + 1 },
        'D' => if D_BORDERS.contains(&p) { p } else if D_TWOS.contains(&p) { p + 2 } else { p + 4 },
        'L' => if L_BORDERS.contains(&p) { p } else { p - 1 },
        _ => panic!()
    }
}

/*
        1
    2   3   4
5   6   7   8   9
    10  11  12      A B C
        13            D

*/

fn main() {
    let lines = io::stdin().lock().lines().collect::<Vec<_>>();
    for line in lines {
        let res = line.unwrap().chars().fold(5, |prev, action| do_action(prev, &action));
        println!("{}", res);
    }
}