use std::io::BufRead;


fn is_valid(chars: Vec<char>, numbers: &Vec<usize>) -> bool {
    &chars.iter().collect::<String>().split('.').filter(|x| x != &"").map(|x| x.chars().count()).collect::<Vec<_>>() == numbers
}

fn find_arrangements(index: usize, max_i: usize, chars: Vec<char>, numbers: &Vec<usize>) -> u32 {
    if index == max_i {
        return if is_valid(chars, numbers) { 1 } else { 0 }
    }
    if chars[index] == '?' {
        let mut a = chars.clone();
        let mut b = chars.clone();
        a[index] = '.';
        b[index] = '#';
        return find_arrangements(index + 1, max_i, a, numbers) + find_arrangements(index + 1, max_i, b, numbers);
    } else {
        return find_arrangements(index + 1, max_i, chars, numbers);
    }
}

fn main() {
    let lines: Vec<Vec<String>> = std::io::stdin().lock().lines().map(|line| line.unwrap().split(" ").map(|s| s.to_string()).collect()).collect();
    let mut sum = 0;
    for splitted in lines {
        let chars: Vec<char> = splitted[0].chars().collect();
        let numbers = splitted[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        sum += find_arrangements(0, chars.len(), chars, &numbers);
    }
    println!("{}", sum);
}