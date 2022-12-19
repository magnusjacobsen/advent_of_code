use std::io::BufRead;

fn get_rock(num: u32, rel_pos: (isize, isize)) -> Vec<(isize, isize)> {
    let rock = match num % 5 {
        0 => vec![(2,0), (3,0), (4,0), (5,0)],
        1 => vec![(3,0), (2,1), (3,1), (4,1), (3,2)],
        2 => vec![(2,0), (3,0), (4,0), (4,1), (4,2)],
        3 => vec![(2,0), (2,1), (2,2), (2,3)],
        4 => vec![(2,0), (3,0), (2,1), (3,1)],
        _ => panic!("WAT?"),
    };
    rock.iter().map(|x| (x.0 + 1, x.1 + rel_pos.1)).collect()
}

fn move_rock(dir: char, rock: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let pos = match dir {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        _ => panic!("WHY ARE WE HERE???!??!!!!!!?"),
    };
    rock.iter().map(|x| (x.0 + pos.0, x.1 + pos.1)).collect()
}

fn collision(rock: &Vec<(isize, isize)>, chamber: &Vec<Vec<u8>>) -> bool {
    rock.iter().any(|x| chamber[x.1 as usize][x.0 as usize] == 1)
}

fn highest_point(rock: Vec<(isize, isize)>) -> isize {
    rock.iter().map(|x| x.1).max().unwrap()
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer).unwrap();
    let winds = buffer.chars().collect::<Vec<_>>();

    let mut chamber = vec![vec![0;9];10000];
    chamber[0] = vec![1;9];
    for i in 0..chamber.len() {
        chamber[i][0] = 1;
        chamber[i][8] = 1;
    }

    let wind_len = winds.len();
    let mut rock_count = 0;
    let mut wind_count = 0;
    let mut bottom = 0;
    while rock_count < 2022 {
        let rel_pos = (0, bottom + 4);
        let mut rock = get_rock(rock_count, rel_pos);
        loop {
            let mut new_rock = move_rock(winds[wind_count % wind_len], rock.clone());
            wind_count += 1;

            if !collision(&new_rock, &chamber) {
                rock = new_rock;
            }

            new_rock = move_rock('^', rock.clone());
            if !collision(&new_rock, &chamber) {
                rock = new_rock;
            } else {
                rock.iter().for_each(|(x, y)| chamber[*y as usize][*x as usize] = 1);
                bottom = bottom.max(highest_point(rock));
                break;
            }
        }
        rock_count += 1;
    }

    println!("{}", bottom);
}