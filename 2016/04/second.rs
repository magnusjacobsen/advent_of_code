use std::io::{self, BufRead};

fn main() {
    io::stdin().lock().lines().for_each(|line| {
        let unwrapped = line.unwrap();
        let splitted = unwrapped.split("-").collect::<Vec<_>>();
        let second_part = splitted[splitted.len() - 1].split("[").collect::<Vec<_>>();
        let sector_id = second_part[0].parse::<u32>().unwrap();
        let out_str: String = unwrapped.chars().map(|x| if x == '-' { ' ' } else if x == '[' || x == ']' || x.is_digit(10) { x } else {std::char::from_u32(((x as u32 - 'a' as u32) + sector_id) % 26 + 'a' as u32).unwrap()}).collect();
        println!("{:?}", out_str);
    });
}