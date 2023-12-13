use std::io::BufRead;

// # = 0 = rock
// . = 1 = ash
fn get_patterns() -> Vec<Vec<Vec<u8>>> {
    let lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line
            .unwrap()
            .chars()
            .map(|x| if x == '#' { 0 } else { 1 })
            .collect::<Vec<_>>()
        ).collect();
    let mut out = vec![];
    let mut current = vec![];
    for line in lines {
        if line.is_empty() {
            out.push(current);
            current = vec![];
            continue;
        }
        current.push(line);
    }
    out.push(current);
    out
}

fn transpose(v: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..v[0].len())
        .map(|i| v
            .iter()
            .map(|inner| inner[i].clone())
            .collect::<Vec<_>>()
        ).collect()
}

fn all_matches(pattern: &Vec<Vec<u8>>, start: usize, len: usize) -> bool {
    for i in 1..start + 1 {
        let right = i + start + 1;
        if right >= len {
            return true;
        }
        if pattern[start - i] != pattern[right] {
            return false;
        }
    }
    true
}

fn matches_horizontal(pattern: &Vec<Vec<u8>>) -> Option<usize> {
    let len = pattern.len();
    for i in 0..len - 1 {
        if pattern[i] == pattern[i + 1] && all_matches(pattern, i, len) {
            return Some(i);
        }
    }
    None
}

fn main() {
    let mut sum = 0;
    for pattern in get_patterns() {
        let vert = transpose(&pattern);
        if let Some(h_val) = matches_horizontal(&pattern) {
            sum += (h_val + 1) * 100; 
        }
        if let Some(v_val) = matches_horizontal(&vert) {
            sum += v_val + 1;
        }
    }
    println!("{}", sum);
}