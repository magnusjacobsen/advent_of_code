use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  let mut possibles: Vec<HashMap<String,String>> = vec![];
  possibles.push(HashMap::new());
  
  while let Some(v) = lines.next() {
    let tmp = v.unwrap();
    if tmp == "" {
      possibles.push(HashMap::new());
    } else {
      let n = possibles.len();
      let split_all: Vec<&str> = tmp.split(" ").map(|x| x).collect();
      for s in split_all {
        let split: Vec<&str> = s.split(":").map(|x| x).collect();
        possibles[n - 1].insert(split[0].to_string(), split[1].to_string());
      }
    }
  }
  
  let mut count = 0;
  for p in possibles {
    if p.len() > 7 {
      count += 1;
    }
    if !p.contains_key("cid") && p.len() > 6 {
      count += 1;
    }
  }
  println!("{}", count);
}
