#![feature(stdin_forwarders)]
fn in_target((x, y): (i32, i32), target: &Vec<Vec<i32>>) -> bool {
    x >= target[0][0] && x <= target[0][1] && y >= target[1][0] && y <= target[1][1]
}

fn main() {
    // vec [[x1, x2], [y1, y2]]
    let target = std::io::stdin().lines().next().unwrap().unwrap().split(": ").last().unwrap().split(", ").map(|x| x.trim().split("=").last().unwrap().split("..").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    println!("{:?}", target);

    let mut max_y = i32::MIN;
    let mut candidate = (0, 0);

    'outer: for x in -100..100 {
        for y in -100..100 {
            let mut vel = (x, y);
            let mut probe = vel;
            let mut highest_y = i32::MIN;
            'steps: loop {// step
                vel = (
                    if vel.0 > 0 { vel.0 - 1 } else if vel.0 < 0 { vel.0 + 1 } else { vel.0 },
                    vel.1 - 1
                );
                highest_y = if probe.1 > highest_y { probe.1 } else { highest_y };
                probe = (probe.0 + vel.0, probe.1 + vel.1);
                if in_target(probe, &target) {
                    if highest_y > max_y {
                        max_y = highest_y;
                        candidate = (x,y)
                    }
                    break 'steps;
                } else if probe.1 < target[1][0] {
                    break 'steps;
                }
            }
        }
    }
    println!("max y: {} vel: {:?}", max_y, candidate)
}