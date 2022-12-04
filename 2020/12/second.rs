use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum In {
  North,
  South,
  East,
  West,
  Left,
  Right,
  Forward,
}

fn parse_input() -> Vec<(In, i32)> {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines().filter(|x| x.as_ref().unwrap() != "");
  let mut instructions: Vec<(In, i32)> = vec![];
  while let Some(v) = lines.next() {
    let line = v.unwrap();
    let instruction = match &line[0..1] {
      "N" => In::North,
      "S" => In::South,
      "E" => In::East,
      "W" => In::West,
      "L" => In::Left,
      "R" => In::Right,
      _   => In::Forward,
    };
    let amount = i32::from_str(&line[1..]).unwrap();
    instructions.push((instruction, amount));
  }
  instructions
}

fn left(waypoint: (i32, i32), degrees: i32) -> (i32, i32) {
  if degrees == 0 {
    return waypoint;
  }
  left((-waypoint.1, waypoint.0), degrees - 90)
}

fn right(waypoint: (i32, i32), degrees: i32) -> (i32, i32) {
  if degrees == 0 {
    return waypoint;
  }
  right((waypoint.1, -waypoint.0), degrees - 90)
}


fn move_waypoint(instruction: In, waypoint: (i32, i32),  amount: i32) -> (i32, i32) {
  match instruction {
    In::North => (waypoint.0, waypoint.1 + amount),
    In::South => (waypoint.0, waypoint.1 - amount),
    In::East => (waypoint.0 + amount, waypoint.1),
    _ => (waypoint.0 - amount, waypoint.1), // always West
  }
}

fn calculate_manhattan(instructions: Vec<(In, i32)>) -> i32 {
  let mut position = (0,0);
  let mut waypoint = (10, 1);
  for (instruction, amount) in instructions {
    match instruction {
      In::Right => { waypoint = right(waypoint, amount); },
      In::Left => { waypoint = left(waypoint, amount); },
      In::Forward => {
        position = (position.0 + waypoint.0 * amount, position.1 + waypoint.1 * amount);
      },
      _ => {
        waypoint = move_waypoint(instruction, waypoint, amount);
      },
    }
  }
  position.0.abs() + position.1.abs()
}

fn main() {
  let instructions = parse_input();
  let result = calculate_manhattan(instructions);
  println!("{}", result);
}
