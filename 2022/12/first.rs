use std::io::BufRead;
use std::collections::VecDeque;

fn legal_move(a: char, b: char) -> bool {
    a == 'S' || (a == 'z' && b == 'E') || (a == 'y' && b == 'E') || b.is_lowercase() && a.is_lowercase() &&  (b as i32) - (a as i32) < 2
}

fn main() {
    let grid = std::io::stdin().lock().lines().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut start = (0, 0, 0);
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                start = (i, j, 0);
                break 'outer;
            }
        }
    }

    let max_y = grid.len();
    let max_x = grid[0].len();
    let mut visited = vec![vec![false; max_x]; max_y];
    let mut queue = VecDeque::from([start]);
    let mut final_steps = 0;

    // 422 too high

    while let Some((i, j, steps)) = queue.pop_front() {

        if !visited[i][j] {
            visited[i][j] = true;
            let elevation = grid[i][j];
            if elevation == 'E' {
                final_steps = steps;
                break;
            }
            println!("i: {}, j: {}, steps: {}, elevation: {}", i, j, steps, elevation);
            // north
            if i > 0 && legal_move(elevation, grid[i - 1][j]) {
                queue.push_back((i - 1, j, steps + 1));
            }
            // east
            if j < max_x - 1 && legal_move(elevation, grid[i][ j + 1]) {
                queue.push_back((i, j + 1, steps + 1));
            }
            // south
            if i < max_y - 1 && legal_move(elevation, grid[i + 1][j]) {
                queue.push_back((i + 1, j, steps + 1));
            }
            // west
            if j > 0 && legal_move(elevation, grid[i][j - 1]) {
                queue.push_back((i, j - 1, steps + 1));
            }
        }
    }

    println!("{}", final_steps);
}