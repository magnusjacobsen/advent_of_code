use std::io::{self, BufRead};

fn parse_input() -> Vec<Vec<String>> {
  io::stdin()
    .lock()
    .lines()
    .filter(|x| x.as_ref().unwrap() != "")
    .map(|y| y
      .unwrap()
      .replace("(", "( ")
      .replace(")", " )")
      .split(" ")
      .map(|z| z.to_string())
      .collect())
    .collect()
}

fn calculate(eq: &Vec<String>, mut index: usize) -> (i64, usize) {
  let mut acc = 0;
  let mut do_add = true;
  while index < eq.len() {
    match eq[index].as_str() {
      "+" => { do_add = true; },
      "*" => {do_add = false; },
      "(" => {
        let (other_number, i) = calculate(eq, index + 1);
        index = i;
        if do_add {
          acc += other_number;
        } else {
          acc *= other_number;
        }
      },
      ")" => {
        return (acc, index);
      },
      x => {
        let parsed = x.parse::<i64>().unwrap();
        if do_add {
          acc += parsed;
        } else {
          acc *= parsed;
        }
      }
    }
    index += 1;
  }
  (acc, index)
}

fn main() {
  let input = parse_input();
  let mut sum = 0;
  for i in 0..input.len() {
    let (result, _) = calculate(&input[i], 0);
    sum += result;
  }
  println!("{}", sum);
}
