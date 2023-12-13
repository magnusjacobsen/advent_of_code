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

fn all_matches(pattern: &Vec<Vec<u8>>, start: usize, height: usize) -> bool {
    for i in 1..start + 1 {
        let right = i + start + 1;
        if right >= height {
            return true;
        }
        if pattern[start - i] != pattern[right] {
            return false;
        }
    }
    true
}

fn find_match(pattern: &Vec<Vec<u8>>, original: Option<usize>, height: usize) -> Option<usize> {
    for i in 0..height - 1 {
        if pattern[i] == pattern[i + 1] && all_matches(pattern, i, height) {
            if let Some(og_val) = original {
                if og_val == i {
                    continue;
                }
            }
            return Some(i);
        }
    }
    None
}

fn find_smudge_matches(pattern: &Vec<Vec<u8>>) -> Option<usize> {
    let height = pattern.len();
    let width = pattern[0].len();
    let original_match = find_match(pattern, None, height);
    for k in 0..height {
        for l in 0..width {
            let mut copy = pattern.clone();
            copy[k][l] = if copy[k][l] == 0 { 1 } else { 0 };
            if let Some(new_line) = find_match(&copy, original_match, height) {
                return Some(new_line);
            }
        }
    }
    None
}

fn main() {
    let mut sum = 0;
    let mut nones = vec![];
    for (i, pattern) in get_patterns().iter().enumerate() {
        let vert = transpose(&pattern);
        if let Some(h_val) = find_smudge_matches(&pattern) {
            sum += (h_val + 1) * 100; 
        }
        if let Some(v_val) = find_smudge_matches(&vert) {
            sum += v_val + 1;
        }
    }
    println!("{}", sum);
}