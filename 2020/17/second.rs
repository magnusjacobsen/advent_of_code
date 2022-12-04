use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};

fn parse_input() -> HashSet<(i32,i32,i32,i32)> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  let mut actives: HashSet<(i32,i32,i32,i32)> = HashSet::new();
  let mut i = 0;
  while let Some(v) = lines.next() {
    let tmp = v.unwrap();
    if tmp == "" {
      continue;
    } else {
      let mut j = 0;
      for c in tmp.chars() {
        if c == '#' {
          let active = (j, i, 0, 0);
          actives.insert(active);
        }
        j += 1;
      }
    }
    i += 1;
  }
  actives
}

fn cycle(actives: HashSet<(i32,i32,i32,i32)>) -> HashSet<(i32,i32,i32,i32)> {
  let mut ping: HashMap<(i32,i32,i32,i32),i32> = HashMap::new();
  for active in &actives {
    for x in active.0 - 1..active.0 + 2 {
      for y in active.1 - 1..active.1 + 2 {
        for z in active.2 - 1..active.2 + 2 {
          for w in active.3 - 1..active.3 + 2 {
            let pinged = (x, y, z, w);
            if pinged == *active {
              continue;
            } else {
              *ping.entry(pinged).or_insert(0) += 1;
            }
          }
        }
      }
    }
  }

  let mut next_cycle: HashSet<(i32,i32,i32,i32)> = HashSet::new();
  for (pinged, count) in ping {
    if (count == 2 && actives.contains(&pinged)) || count == 3 {
      next_cycle.insert(pinged);
    }
  }
  next_cycle
}

fn main() {
  let mut actives = parse_input();
  for _ in 0..6 {
    actives = cycle(actives);
  }

  println!("{}", actives.len());
}
