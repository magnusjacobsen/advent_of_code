use std::io::BufRead;
use std::collections::HashSet;

#[inline(always)]
fn manhattan(sensor: (i32, i32), beacon: (i32, i32)) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

// today we bruteforce, tomorrow we sleep
fn main() {
    let sensors = std::io::stdin().lock().lines().map(|line| {
        let unwrapped = line.unwrap();
        let mut spl1 = unwrapped.split(": closest beacon is at x=");
        let mut spl2 = spl1.next().unwrap().split("x=");
        let mut spl3 = spl2.nth(1).unwrap().split(", y=");
        let sensor_x = spl3.next().unwrap().parse().unwrap();
        let sensor_y = spl3.next().unwrap().parse().unwrap();
        let mut spl4 = spl1.next().unwrap().split(", y=");
        let beacon_x = spl4.next().unwrap().parse().unwrap();
        let beacon_y = spl4.next().unwrap().parse().unwrap();
        ((sensor_x, sensor_y), (beacon_x, beacon_y))
    }).collect::<Vec<((i32, i32), (i32, i32))>>();

    let mut beacons = HashSet::new();
    let mut candidates = HashSet::new();
    let max_width = 4000000;

    for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in sensors.clone() {
        let man_dist = manhattan((sensor_x, sensor_y), (beacon_x, beacon_y)) + 1;
        beacons.insert((beacon_x, beacon_y));

        for i in -man_dist..=man_dist {
            let y = sensor_y + i;
            if y < 0 || y > max_width {
                continue;
            }
            let x1 = sensor_x + (i + if i < 0 {man_dist} else {-man_dist});
            let x2 = sensor_x - (i + if i < 0 {man_dist} else {-man_dist});

            if x1 >= 0 && x1 <= max_width {
                candidates.insert((x1, y));
            }
            if x2 >= 0 && x2 <= max_width {
                candidates.insert((x2, y));
            }
        }
    }
    
    let mut real_candidates: Vec<(i32, i32)> = candidates.into_iter().collect();
    for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in sensors {
        let man_dist = manhattan((sensor_x, sensor_y), (beacon_x, beacon_y));
        real_candidates = real_candidates.into_iter().filter(|(x, y)| manhattan((sensor_x, sensor_y), (*x, *y)) > man_dist).collect();
    }

    let distress = real_candidates[0];
    let freq = distress.0 as i128 * 4000000_i128 + distress.1 as i128;
    println!("{}", freq);
}