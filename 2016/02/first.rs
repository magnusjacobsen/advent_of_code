use std::io::{self, BufRead};

fn do_action(prev: i32, action: &char) -> i32 {
    match action {
        'U' => if prev < 4 { prev } else { prev - 3 },
        'R' => if prev == 3 || prev == 6 || prev == 9 { prev } else { prev + 1 },
        'D' => if prev > 6 { prev } else { prev + 3 },
        'L' => if prev == 1 || prev == 4 || prev == 7 { prev } else { prev - 1 },
        _ => panic!()
    }
}

fn main() {
    let lines = io::stdin().lock().lines().collect::<Vec<_>>();
    for line in lines {
        let res = line.unwrap().chars().fold(5, |prev, action| do_action(prev, &action));
        println!("{}", res);
    }
}