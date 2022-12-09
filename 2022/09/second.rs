use std::io::BufRead;
use std::collections::HashSet;

fn touches(a: (i32, i32), b: (i32, i32)) -> bool {
    match (a.0 - b.0, a.1 - b.1) {
        (-1, -1)    => true,
        (-1, 0)     => true,
        (-1, 1)     => true,
        (0, -1)     => true,
        (0, 0)      => true,
        (0, 1)      => true,
        (1, -1)     => true,
        (1, 0)      => true,
        (1, 1)      => true,
        (_, _)      => false,
    }
}
// 4968 too high
fn main() {
    let lines = std::io::stdin().lock().lines().map(|line| {
        let line2 = line.unwrap();
        let split = line2.split(" ").collect::<Vec<_>>();
        let dir = split[0].chars().next().unwrap();
        let steps: i32 = split[1].parse().unwrap();
        (dir, steps)
    }).collect::<Vec<_>>();
    let mut knots = vec![(0,0); 10];
    let mut visited = HashSet::new();
    visited.insert((0,0));

    for (dir, steps) in lines {
        let vec = match dir {
            'R' => (0, 1),
            'U' => (-1, 0),
            'L' => (0, -1),
            'D' => (1, 0),
            _ => panic!("should not end here"),
        };
        for _ in 0..steps {
            knots[0] = (knots[0].0 + vec.0, knots[0].1 + vec.1);

            for i in 1..10 {
                if !touches(knots[i - 1], knots[i]) {
                    let mov = get_mov(knots[i - 1], knots[i]);
                    knots[i] = (knots[i].0 + mov.0, knots[i].1 + mov.1);
                }
            }
            visited.insert(knots[9]);
        }
    }
    println!("{}", visited.len());
}

fn get_mov(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    if h == t {
        (0, 0)
    } else if h.0 == t.0 {
        if h.1 > t.1 {
            (0, 1)
        } else {
            (0, -1)
        }
    } else if h.1 == t.1 {
        if h.0 > t.0 {
            (1, 0)
        } else {
            (-1, 0)
        }
    } else if h.0 > t.0 {
        if h.1 > t.1 {
            (1, 1)
        } else {
            (1, -1)
        }
    } else { // h.0 < t.0
        if h.1 > t.1 {
            (-1, 1)
        } else {
            (-1, -1)
        }
    }
}