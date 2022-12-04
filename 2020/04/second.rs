use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::str::FromStr;
use std::iter::FromIterator;

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

  let allowed_colors: HashSet<char> = HashSet::from_iter("0123456789abcdef".chars());
  let mut count = 0;
  'a: for p in possibles {
    if p.len() > 7 || (!p.contains_key("cid") && p.len() > 6) {
      // byr, birth year
      let byr = usize::from_str(&p["byr"]).unwrap_or(0);
      if byr < 1920 || byr > 2002 { continue 'a; }
      
      // iyr, issue year
      let iyr = usize::from_str(&p["iyr"]).unwrap_or(0);
      if iyr < 2010 || iyr > 2020 { continue 'a; }

      // eyr, expiration year
      let eyr = usize::from_str(&p["eyr"]).unwrap_or(0);
      if eyr < 2020 || eyr > 2030 { continue 'a; }

      // hgt, height
      let hgt = &p["hgt"];
      if hgt.contains("in") {
        let s: String = hgt.chars().take(2).collect();
        let h = i32::from_str(&s).unwrap_or(0);
        if h < 59 || h > 76 { continue 'a; }
      } else if hgt.contains("cm") {
        let s: String = hgt.chars().take(3).collect();
        let h = i32::from_str(&s).unwrap_or(0);
        if h < 150 || h > 193 { continue 'a; }
      } else {
        continue 'a;
      }

      // hcl, hair color
      let mut hcl = p["hcl"].to_string();
      if hcl.contains("#") {
        hcl.remove(0);
        if hcl.len() != 6 { continue 'a; }
        for c in hcl.chars() {
          if !allowed_colors.contains(&c) { continue 'a; }
        }
      } else { continue 'a; }

      // ecl, eye color
      let ecl = &p["ecl"];
      if ecl != "amb" && ecl != "blu" && ecl != "brn" && ecl != "gry" && ecl != "grn" && ecl != "hzl" && ecl != "oth" {
        continue 'a;
      }

      // pid, passport id
      let pid = &p["pid"];
      let pid2 = i32::from_str(pid).unwrap_or(-1);
      if pid.len() != 9 && pid2 < 0 {
        continue 'a;
      }
      
      count += 1;
    }
  }
  println!("{}", count);
  println!("wtf, not the correct number. In my case it was one above the right one");
  println!("probably should have used a regex crate for this :D")
}
