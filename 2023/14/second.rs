use std::io::BufRead;
use std::collections::HashMap;

fn get_dish() -> Vec<Vec<u8>> {
    std::io::stdin().lock().lines().map(|line| line.unwrap().chars().map(|x| if x == 'O' {0} else if x == '.' {1} else {2}).collect::<Vec<u8>>()).collect::<Vec<_>>()
}

fn swap(dish: &mut Vec<Vec<u8>>, i: usize, ii: usize, j: usize, jj: usize) {
    let tmp = dish[i][j];
    dish[i][j] = dish[ii][jj];
    dish[ii][jj] = tmp;
}

fn move_north(dish: &mut Vec<Vec<u8>>) {
    (0..dish.len()).for_each(|i| (0..dish[0].len()).for_each(|j|
        if dish[i][j] == 0 {
            let mut new_spot = i;
            'a: for ii in (0..i).rev() {
                if dish[ii][j] != 1 {
                    break 'a
                }
                new_spot = ii;
            }
            swap(dish, i, new_spot, j, j);
        }
    ));
}

fn move_south(dish: &mut Vec<Vec<u8>>) {
    (0..dish.len()).rev().for_each(|i| (0..dish[0].len()).rev().for_each(|j|
        if dish[i][j] == 0 && i < dish.len() - 1 {
            let mut new_spot = i;
            'a: for ii in i + 1..dish.len() {
                if dish[ii][j] != 1 {
                    break 'a
                }
                new_spot = ii;
            }
            swap(dish, i, new_spot, j, j);
        }
    ));
}

fn move_west(dish: &mut Vec<Vec<u8>>) {
    (0..dish.len()).for_each(|i| (0..dish[0].len()).for_each(|j|
        if dish[i][j] == 0 {
            let mut new_spot = j;
            'a: for jj in (0..j).rev() {
                if dish[i][jj] != 1 {
                    break 'a
                }
                new_spot = jj;
            }
            swap(dish, i, i, j, new_spot);
        }
    ));
}

fn move_east(dish: &mut Vec<Vec<u8>>) {
    (0..dish.len()).rev().for_each(|i| (0..dish[0].len()).rev().for_each(|j|
        if dish[i][j] == 0 && j < dish[0].len() - 1 {
            let mut new_spot = j;
            'a: for jj in j + 1..dish[0].len() {
                if dish[i][jj] != 1 {
                    break 'a
                }
                new_spot = jj;
            }
            swap(dish, i, i, j, new_spot);
        }
    ));
}

fn get_load(dish: &Vec<Vec<u8>>) -> usize {
    (0..dish.len()).fold(0, |sum, i| sum + (0..dish[0].len()).fold(0, |sum2, j| sum2 + if dish[i][j] == 0 {dish.len() - i} else {0}))
}

fn main() {
    let mut dish = get_dish();
    let mut memory = HashMap::new();
    let max = 1_000_000_000;
    let mut i = 1;
    let mut looking_for_cycle = true; 
    while i <= max {
        move_north(&mut dish);
        move_west(&mut dish);
        move_south(&mut dish);
        move_east(&mut dish);
        if looking_for_cycle {
            if let Some(old_i) = memory.get(&dish) {
                let cycle_len = i - old_i;
                i = ((max - old_i) / cycle_len) * cycle_len + old_i;
                looking_for_cycle = false;
            } else {
                memory.insert(dish.clone(), i);
            }
        }
        i += 1;
    }
    let load = get_load(&dish);
    println!("{}", load);
}