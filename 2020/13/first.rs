use std::io::{self, BufRead};
use std::str::FromStr;

fn parse_input() -> (usize, Vec<usize>) {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines().filter(|x| x.as_ref().unwrap() != "");
  let earliest = usize::from_str(&lines.next().unwrap().unwrap()).unwrap();
  let busses: Vec<usize> = lines.next().unwrap().unwrap()
    .split(",")
    .filter(|x| x != &"x")
    .map(|x| usize::from_str(x).unwrap())
    .collect();
  (earliest, busses)
}

fn main() {
  let (earliest, busses) = parse_input();
  let mut answer = 0;
  let mut min = std::usize::MAX;
  for bus in busses {
    let wait = bus - ((earliest + bus) % bus);
    if wait < min {
      min = wait;
      answer = wait * bus;
    }
  }
  println!("{}", answer);
}
