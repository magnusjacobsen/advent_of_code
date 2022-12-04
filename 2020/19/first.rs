use std::io::{self, BufRead};
use std::collections::HashMap;

fn parse_input() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  let mut char_rules: HashMap<usize,char> = HashMap::new(); // rule_id to char_id
  let mut combination_rules: HashMap<usize,Vec<Vec<usize>>> = HashMap::new(); // rule_id to other_rules
  let mut collected_rules: Vec<String> = vec![];
  'a: while let Some(v) = lines.next() {
    let tmp = v.unwrap();
    if tmp == "" { break 'a; }
    collected_rules.push(tmp.to_string());
  }

  'b: for r in &collected_rules {
    if r.contains("\"") {
      let splitted: Vec<&str> = r.split(": ").collect();
      let id = splitted[0].parse::<usize>().unwrap();
      let c = splitted[1].chars().nth(1).unwrap();
      char_rules.insert(id, c);
    } else if r.contains("|") {
      let splitted: Vec<Vec<usize>> = r
        .split(" | ")
        .map(|x| x
          .split(" ")
          .map(|y| y
            .parse::<usize>()
            .unwrap())
            .collect()
          .collect())
    }
  }
  for (c,id) in char_rules {
    println!("c: {}, id: {}", c, id);
  }
}

fn main() {
  parse_input();
}
