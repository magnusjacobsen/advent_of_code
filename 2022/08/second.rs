use std::io::BufRead;

fn main() {
    let grid = std::io::stdin().lock().lines().map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let len_y = grid.len();
    let len_x = grid[0].len();
    let mut distances = vec![];

    for i in 1..len_y - 1 {
        for j in 1..len_x - 1 {
            let tree = grid[i][j];
            let mut dis_north = 0;
            let mut dis_east = 0;
            let mut dis_south = 0;
            let mut dis_west = 0;
            // north
            for y in (0..i).rev() {
                dis_north += 1;
                if grid[y][j] >= tree {
                    break;
                }
            }
            // east
            for x in j + 1..len_x {
                dis_east += 1;
                if grid[i][x] >= tree {
                    break;
                }
            }
            // south
            for y in i + 1..len_y {
                dis_south += 1;
                if grid[y][j] >= tree {
                    break;
                }
            }
            // west
            for x in (0..j).rev() {
                dis_west += 1;
                if grid[i][x] >= tree {
                    break;
                }
            }
            let distance = dis_north * dis_east * dis_south * dis_west;
            //println!("{}, {}, {}, {} ,{}", distance, dis_north, dis_east, dis_south, dis_west);
            distances.push(distance);
        }
    }
    println!("{:?}", distances.iter().max().unwrap());
}