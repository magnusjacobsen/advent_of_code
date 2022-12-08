use std::io::BufRead;

fn main() {
    let grid = std::io::stdin().lock().lines().map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let len_y = grid.len();
    let len_x = grid[0].len();
    let mut num_visible = 0;

    for i in 1..len_y - 1 {
        for j in 1..len_x - 1 {
            let tree = grid[i][j];
            let mut vis_north = true;
            let mut vis_east = true;
            let mut vis_south = true;
            let mut vis_west = true;
            // north
            for y in 0..i {
                if grid[y][j] >= tree {
                    vis_north = false;
                    break;
                }
            }
            // east
            for x in j + 1..len_x {
                if grid[i][x] >= tree {
                    vis_east = false;
                    break;
                }
            }
            // south
            for y in i + 1..len_y {
                if grid[y][j] >= tree {
                    vis_south = false;
                    break;
                }
            }
            // west
            for x in 0..j {
                if grid[i][x] >= tree {
                    vis_west = false;
                    break;
                }
            }
            if vis_north || vis_east || vis_south || vis_west {
                num_visible += 1;
            }
        }
    }

    let num_edge = len_x * 2 + len_y * 2 - 4;
    println!("{}", num_visible + num_edge);
}