use std::io::{self,BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut count = 0;

    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" {
            continue;
        }
        let split: Vec<&str> = tmp.split(" ").collect();
        let fst = isize::from_str(split[0].split("-").next().unwrap()).unwrap();
        let snd = isize::from_str(split[0].split("-").take(2).last().unwrap()).unwrap();
        let character: &str = &split[1][0..1];
        let password: &str = split[2];
        let fst_c = &password[(fst-1) as usize..fst as usize];
        let snd_c = &password[(snd-1) as usize..snd as usize];
        if (character == fst_c && character != snd_c) ||
            character != fst_c && character == snd_c { 
            count += 1;
        }
    }
    println!("{:?}", count);
}