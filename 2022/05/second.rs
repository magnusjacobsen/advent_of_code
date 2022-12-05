use std::io::BufRead;

fn main() {
    /*
        Parsing
    */
    let mut lines = std::io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut ins = vec![];
    loop {
        let line = lines.pop().unwrap();
        if line == "" {
            break;
        }
        let splitted = line.split(" ").collect::<Vec<_>>();
        let amount = usize::from_str_radix(splitted[1], 10).unwrap();
        let from = usize::from_str_radix(splitted[3], 10).unwrap() - 1;
        let to = usize::from_str_radix(splitted[5], 10).unwrap() - 1;
        ins.push((amount, from, to));
    }
    let num_cols = lines.pop().unwrap().chars().rev().collect::<Vec<_>>()[1].to_digit(10).unwrap() as usize;
    
    let mut cols = vec![vec![]; num_cols];
    while let Some(line) = lines.pop() {
        let cs = line.chars().collect::<Vec<_>>();
        for i in 0..num_cols {
            let idx = 1 + i * 4;
            if cs[idx].is_alphabetic() {
                cols[i].push(cs[idx]);
            }
        } 
    }

    /*
        Solving
    */
    while let Some((amount, from, to)) = ins.pop() {
        let mut tmp = vec![];
        for _i in 0..amount {
            let crt = cols[from].pop().unwrap();
            tmp.push(crt);
        }
        for _i in 0..amount {
            let crt = tmp.pop().unwrap();
            cols[to].push(crt);
        }
    }

    let res = (0..num_cols).map(|j| cols[j].pop().unwrap()).collect::<String>();
    println!("{:?}", res);
}
