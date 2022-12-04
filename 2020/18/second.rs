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
  let mut do_add = false;
  let mut to_multiply = vec![];
  'a: while index < eq.len() {
    match eq[index].as_str() {
      "+" => { do_add = true; },
      "*" => { do_add = false;},
      "(" => {
        let (other_number, i) = calculate(eq, index + 1);
        index = i;
        if do_add { 
          let last = to_multiply.len() - 1;
          to_multiply[last] += other_number; 
        } else { 
          to_multiply.push(other_number); 
        }
      },
      ")" => { 
        break 'a; 
      },
      x => {
        let parsed = x.parse::<i64>().unwrap();
        if do_add { 
          let last = to_multiply.len() - 1;
          to_multiply[last] += parsed; 
        } else { 
          to_multiply.push(parsed); 
        }
      }
    }
    index += 1;
  }
  (to_multiply.iter().fold(1, |acc, x| acc * x), index)
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
