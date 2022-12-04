use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> Vec<(i64, i64)> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines().filter(|x| x.as_ref().unwrap() != "");
  let _earliest = lines.next();
  let tmp = lines.next().unwrap().unwrap();
  let splitted: Vec<&str> = tmp.split(",").collect();
  let mut busses: Vec<(i64, i64)> = vec![];
  for i in 0..splitted.len() {
    match splitted[i] {
      "x" => {},
      x => busses.push((i as i64, i64::from_str(x).unwrap())),
    }
  }
  busses
}

fn main() {
  let mut busses = parse_input();
  let variables = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t"];
  for i in 0..busses.len() {
    print!("{{y = {}{} - {} }} ",busses[i].1, variables[i], busses[i].0)
  }
  println!("");
  println!("plot the above into wolfram alpha XD");
}
