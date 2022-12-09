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

fn main() {
    let lines = std::io::stdin().lock().lines().map(|line| {
        let line2 = line.unwrap();
        let split = line2.split(" ").collect::<Vec<_>>();
        let dir = split[0].chars().next().unwrap();
        let steps: i32 = split[1].parse().unwrap();
        (dir, steps)
    }).collect::<Vec<_>>();
    let mut tail = (0,0);
    let mut head = (0,0);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for (dir, steps) in lines {
        let vec = match dir {
            'R' => (0, 1),
            'U' => (-1, 0),
            'L' => (0, -1),
            'D' => (1, 0),
            _ => panic!("should not end here"),
        };
        for _i in 0..steps {
            let old_head = head;
            head = (head.0 + vec.0, head.1 + vec.1);
            if !touches(head, tail) {
                tail = old_head;
                visited.insert(tail);
            }
        }
    }
    println!("{}", visited.len());
}