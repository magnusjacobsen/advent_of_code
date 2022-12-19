use std::io::BufRead;
use std::collections::HashMap;

#[inline(always)]
fn get_rock(num: u64, y: usize) -> Vec<(usize, usize)> {
    match num % 5 {
        0 => vec![(3, y), (4, y), (5, y), (6, y)],
        1 => vec![(4, y), (3, 1 + y), (4, 1 + y), (5, 1 + y), (4, 2 + y)],
        2 => vec![(3, y), (4, y), (5, y), (5, 1 + y), (5, 2 + y)],
        3 => vec![(3, y), (3, 1 + y), (3, 2 + y), (3, 3 + y)],
        4 => vec![(3, y), (4, y), (3, 1 + y), (4, 1 + y)],
        _ => panic!("WAT?"),
    }
}

#[inline(always)]
fn move_left(rock: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    rock.iter().map(|(x, y)| (*x - 1, *y)).collect()
}

#[inline(always)]
fn move_right(rock: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    rock.iter().map(|(x, y)| (*x + 1, *y)).collect()
}

#[inline(always)]
fn move_up(rock: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    rock.iter().map(|(x, y)| (*x, *y - 1)).collect()
}

#[inline(always)]
fn collision(rock: &Vec<(usize, usize)>, chamber: &Vec<Vec<u8>>) -> bool {
    rock.iter().any(|(x, y)| chamber[*y][*x] == 1)
}

#[inline(always)]
fn highest_point(rock: &Vec<(usize, usize)>) -> usize {
    rock.iter().map(|(_, y)| *y).max().unwrap()
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer).unwrap();
    let winds = buffer.chars().collect::<Vec<_>>();

    let empty = vec![1,0,0,0,0,0,0,0,1];
    let full_rack = vec![1;9];
    let max_height = 100000;
    let mut chamber = vec![empty.clone(); max_height];
    chamber[0] = full_rack.clone();

    let mut rock_count = 0;
    let mut wind_count = 0;
    let mut rock_bottom = 0;
    let num_rocks = 1000000000000;
    let mut patterns: HashMap<Vec<Vec<u8>>, (u64, usize)> = HashMap::new();

    while rock_count < num_rocks {
        (rock_bottom, wind_count) = drop_rock(rock_count, rock_bottom, wind_count, &winds, &mut chamber);
        rock_count += 1;
        if rock_bottom > 4000 {
            if let Some((start_rock, other_bottom)) = patterns.get(&chamber[rock_bottom-39..= rock_bottom].to_vec()) {
                let diff_rock = rock_count - start_rock;
                let diff_bottom = rock_bottom - other_bottom;
                let rock_mult = (num_rocks - start_rock) / diff_rock;
                rock_count = start_rock + rock_mult * diff_rock;
                
                while rock_count < num_rocks {
                    (rock_bottom, wind_count) = drop_rock(rock_count, rock_bottom, wind_count, &winds, &mut chamber);
                    rock_count += 1;
                }

                println!("{}", rock_bottom - diff_bottom + diff_bottom * rock_mult as usize);
                break;
            } else {
                patterns.insert(chamber[rock_bottom-39..=rock_bottom].into(), (rock_count, rock_bottom));
            }
        }
    }
}

fn drop_rock(rock_count: u64, mut rock_bottom: usize, mut wind_count: usize, winds: &Vec<char>, chamber: &mut Vec<Vec<u8>>) -> (usize, usize) {
    let mut rock = get_rock(rock_count, rock_bottom + 4);
    loop {
        let mut new_rock = if winds[wind_count]
        == '<' {
            move_left(&rock)
        } else {
            move_right(&rock)
        };
        wind_count = if wind_count == winds.len() - 1 { 0 } else { wind_count + 1 };

        if !collision(&new_rock, &chamber) {
            rock = new_rock;
        }

        new_rock = move_up(&rock);
        if !collision(&new_rock, &chamber) {
            rock = new_rock;
        } else {
            rock.iter().for_each(|(x, y)| chamber[*y][*x] = 1);
            rock_bottom = rock_bottom.max(highest_point(&rock));
            break;
        }
    }
    (rock_bottom, wind_count)
}