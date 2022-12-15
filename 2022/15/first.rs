use std::io::BufRead;
use std::collections::HashSet;

fn manhattan(sensor: (i32, i32), beacon: (i32, i32)) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

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

    let mut cannot = HashSet::new();
    let mut beacons = vec![];
    let y = 2000000;

    for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in sensors {
        let man_dist = manhattan((sensor_x, sensor_y), (beacon_x, beacon_y));
        beacons.push((beacon_x, beacon_y));

        let min_y = sensor_y - man_dist;
        let max_y = sensor_y + man_dist;
        if min_y > y || max_y < y {
            continue;
        }
        for x in sensor_x - man_dist..=sensor_x + man_dist {
            if man_dist >= manhattan((sensor_x, sensor_y), (x, y)) {
                cannot.insert((x, y));
            }
        }
    }

    for beacon in beacons {
        cannot.remove(&beacon);
    }

    let in_row = cannot.iter().fold(0_u32, |acc, (_x, yy)| if yy == &y { acc + 1} else {acc});
    println!("{}", in_row);
}