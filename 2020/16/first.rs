use std::io::{self, BufRead};

fn parse_input() -> (Vec<Vec<i32>>, Vec<i32>, Vec<Vec<i32>>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut rules: Vec<Vec<i32>> = vec![];

    // getting all the rules
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" { break; }
        let ranges: Vec<i32> = tmp
            .split(": ")
            .collect::<Vec<&str>>()[1]
            .split(" or ")
            .map(|x| x
                .split("-")
                .map(|y| y
                    .parse::<i32>()
                    .unwrap())
                .collect::<Vec<i32>>())
            .flatten()
            .collect();
        rules.push(ranges);
    }

    // getting my own ticket
    lines.next();
    let my_ticket: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    // scanning other people's tickets
    lines.next();
    lines.next();
    let mut tickets: Vec<Vec<i32>> = vec![];
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        tickets.push(tmp.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
    }
    (rules, my_ticket, tickets)
}

fn sum_invalid_fields(rules: Vec<Vec<i32>>, tickets: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for i in 0..tickets.len() {
        'a: for j in 0..tickets[0].len() {
            let value = tickets[i][j];
            for k in 0..rules.len() {
                if (value >= rules[k][0] && value <= rules[k][1]) || (value >= rules[k][2] && value <= rules[k][3]) {
                    continue 'a;
                }
            }
            sum += value;
            //println!("{} is invalid", value);
        }
    }
    sum
}

fn main() {
    let (rules, _my_ticket, tickets) = parse_input();
    let result = sum_invalid_fields(rules, tickets);
    println!("{}", result);
}