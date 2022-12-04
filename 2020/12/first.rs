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

fn left(direction: In, degrees: i32) -> In {
  if degrees == 0 {
    return direction;
  }
  match direction {
    In::North => left(In::West, degrees - 90),
    In::West => left(In::South, degrees - 90),
    In::South => left(In::East, degrees - 90),
    _ => left(In::North, degrees - 90),
  }
}

fn right(direction: In, degrees: i32) -> In {
  if degrees == 0 {
    return direction;
  }
  match direction {
    In::North => right(In::East, degrees - 90),
    In::East => right(In::South, degrees - 90),
    In::South => right(In::West, degrees - 90),
    _ => right(In::North, degrees - 90),
  }
}


fn move_ship(direction: In, amount: i32) -> (i32, i32) {
  match direction {
    In::North => (amount, 0),
    In::South => (-amount, 0),
    In::East => (0, amount),
    _ => (0, -amount),
  }
}

fn calculate_manhattan(instructions: Vec<(In, i32)>) -> i32 {
  let mut position = (0,0);
  let mut direction = In::East;
  for (instruction, amount) in instructions {
    match instruction {
      In::Right => { direction = right(direction, amount); },
      In::Left => { direction = left(direction, amount); },
      In::Forward => {
        let movement = move_ship(direction, amount);
        position.0 += movement.0;
        position.1 += movement.1;
      },
      _ => {
        let movement = move_ship(instruction, amount);
        position.0 += movement.0;
        position.1 += movement.1;
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
