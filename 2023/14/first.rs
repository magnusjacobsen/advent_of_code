use std::io::BufRead;

fn get_dish() -> Vec<Vec<u8>> {
    std::io::stdin().lock().lines().map(|line| line.unwrap().chars().map(|x| if x == 'O' {0} else if x == '.' {1} else {2}).collect::<Vec<u8>>()).collect::<Vec<_>>()
}

fn move_north_inplace(dish: &mut Vec<Vec<u8>>) {
    (0..dish.len()).for_each(|i| (0..dish[0].len()).for_each(|j|
        if dish[i][j] == 0 {
            let mut new_spot = i;
            'a: for ii in (0..i).rev() {
                if dish[ii][j] != 1 {
                    break 'a
                }
                new_spot = ii;
            }
            let tmp = dish[i][j];
            dish[i][j] = dish[new_spot][j];
            dish[new_spot][j] = tmp;
        }
    ));
}

fn get_load(dish: &Vec<Vec<u8>>) -> usize {
    (0..dish.len()).fold(0, |sum, i| sum + (0..dish[0].len()).fold(0, |sum2, j| sum2 + if dish[i][j] == 0 {dish.len() - i} else {0}))
}

fn main() {
    let mut dish = get_dish();
    move_north_inplace(&mut dish);
    let load = get_load(&dish);
    println!("{}", load);
}