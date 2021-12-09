#![feature(stdin_forwarders)]

// sets up to 7 bits, depending on which chars a-g are present in the str
fn bits(a: &str) -> u8 {
    a.chars().fold(0, |output, c| output | (1 << (c as u8 - 97)))
}

fn is_subpattern(pattern: u8, sub: u8) -> bool {
   (pattern & sub) == sub
}

fn main() { 
   println!("{}", std::io::stdin().lines().map(|line| {
      let parsed = line.unwrap().split(" | ").map(|x| x.split(" ").map(|y| (bits(&y), y.chars().count()) ).collect() ).collect::<Vec<Vec<_>>>();
      
      let mut decided = vec![0; 10];
      let mut undecided = vec![];

      // decide unique length numbers: 1, 4, 7, and 8
      for i in 0..10 {
         let len = parsed[0][i].1;
         if len == 2 { // number 1
            decided[1] = parsed[0][i].0;
         } else if len == 4 { // number 4
            decided[4] = parsed[0][i].0;
         } else if len == 3 { // number 7
            decided[7] = parsed[0][i].0;
         } else if len == 7 { // number 8
            decided[8] = parsed[0][i].0;
         } else {
            undecided.push(parsed[0][i]);
         }
      }

      // decide the rest: 9, 0, 3, 6, 5, and 2
      for (number, length) in undecided {
         if is_subpattern(number, decided[1]) && is_subpattern(number, decided[7]) {
            if is_subpattern(number, decided[4]) {
               decided[9] = number;
            } else if length == 6 { 
               decided[0] = number;
            } else {
               decided[3] = number;
            }
         } else if length == 6 {
            decided[6] = number;
         } else if (number & decided[4]).count_ones() == 3 {
            decided[5] = number;
         } else {
            decided[2] = number;
         }
      }

      let mut s = String::new();

      'outer: for i in 0..4 {
         for j in 0..10 {
            if parsed[1][i].0 == decided[j] {
               s.push(std::char::from_digit(j as u32, 10).unwrap());
               continue 'outer;
            }
         }
      }
      s.parse::<usize>().unwrap()
   }).sum::<usize>());
}