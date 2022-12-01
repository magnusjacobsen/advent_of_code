use std::io::{self, BufRead};

fn main() {
    let tup = io::stdin().lock().lines().fold((0, 0, 0, 0), |acc, current| match current.unwrap().as_str() {
        "" => {
            if acc.3 > acc.0 {
                (acc.3, acc.0, acc.1, 0)
            } else if acc.3 > acc.1 {
                (acc.0, acc.3, acc.1, 0)
            } else if acc.3 > acc.2 {
                (acc.0, acc.1, acc.3, 0)
            } else {
                (acc.0, acc.1, acc.2, 0)
            }
        },        
        num => (acc.0, acc.1, acc.2, u32::from_str_radix(&num, 10).unwrap() + acc.3),
    });
    let res = if tup.3 > tup.0 {
        tup.3 + tup.0 + tup.1
    } else if tup.3 > tup.1 {
        tup.0 + tup.3 + tup.1
    } else if tup.3 > tup.2 {
        tup.0 + tup.1 + tup.3
    } else {
        tup.0 + tup.1 + tup.2
    };
    println!("{}", res);
}