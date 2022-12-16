use std::io::BufRead;
use std::collections::HashSet;

#[inline(always)]
fn manhattan(sensor: (i32, i32), beacon: (i32, i32)) -> i32 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}
// today we bruteforce, tomorrow we sleep
fn main() {
    let max_width = 4000000;
    let mut beacons = HashSet::new();
    let mut sensors = Vec::new();
    
    std::io::stdin().lock().lines().map(|x| x.unwrap()).for_each(|x| {
        let mut parts = x.split_whitespace().filter_map(|word| word.trim_end_matches(&[',', ':']).split("=").nth(1).map(|n| n.parse().unwrap()));
        let sensor = (parts.next().unwrap(), parts.next().unwrap());
        let beacon = (parts.next().unwrap(), parts.next().unwrap());
        let man_dist = manhattan(sensor, beacon);
        beacons.insert(beacon);
        sensors.push((sensor, man_dist));
    });

    for ((sensor_x, sensor_y), man_dist) in sensors.clone() {
        let outer = man_dist + 1;

        let start = if sensor_y - outer < 0 { -sensor_y } else { -outer };
        let finish = if sensor_y + outer > max_width { max_width - sensor_y } else { outer };
        
        for i in start..=finish {
            let y = sensor_y + i;
            let x1 = sensor_x + (i + if i < 0 { outer } else { -outer });
            let x2 = sensor_x - (i + if i < 0 { outer } else { -outer });

            if x1 >= 0 && x1 <= max_width {
                if sensors.iter().all(|(coord, md)| manhattan(*coord, (x1, y)) > *md) {
                    println!("{}", x1 as i64 * 4000000_i64 + y as i64);
                    return;
                }
            }

            if x2 >= 0 && x2 <= max_width {
                if sensors.iter().all(|(coord, md)| manhattan(*coord, (x2, y)) > *md) {
                    println!("{}", x2 as i64 * 4000000_i64 + y as i64);
                    return;
                }
            }
        }
    }
}