use std::io::{self, BufRead};

fn main() {
    let bits: Vec<u64> = io::stdin().lock().lines().map(|x| u64::from_str_radix(&x.unwrap(), 2).unwrap()).collect();
    /* 
        lol this is just plain unreadable.
        The idea is to do something similar as in first.rs, but for two copies of the list
        eat each bit position, starting from the left, we filter the list according to the rules, if the length is more than 1
    */
    println!("{}", (0..64 - bits.iter().max().unwrap().leading_zeros() as usize).rev().fold(vec![bits.clone(), bits.clone()], |ox_co2, i| vec![ if ox_co2[0].len() > 1 { if ox_co2[0].iter().fold(0, |acc, num| if (num & (1 << i)) != 0 { acc + 1 } else { acc - 1 } ) >= 0 { ox_co2[0].iter().filter(|x| *x & (1 << i) != 0).map(|y| *y).collect() } else { ox_co2[0].iter().filter(|x| *x & (1 << i) == 0).map(|y| *y).collect() }} else { ox_co2[0].clone() }, if ox_co2[1].len() > 1 { if ox_co2[1].iter().fold(0, |acc, num| if (num & (1 << i)) != 0 { acc + 1 } else { acc - 1 } ) < 0 { ox_co2[1].iter().filter(|x| *x & (1 << i) != 0).map(|y| *y).collect() } else { ox_co2[1].iter().filter(|x| *x & (1 << i) == 0).map(|y| *y).collect() }} else { ox_co2[1].clone() }]).iter().fold(1, |acc, x| acc * x[0]));
}
