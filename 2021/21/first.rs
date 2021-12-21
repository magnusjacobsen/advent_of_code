#![feature(stdin_forwarders)]

fn main() {
    let mut input = std::io::stdin().lines().map(|x| x.unwrap().split(": ").collect::<Vec<_>>()[1].parse::<u128>());
    let mut p1_pos = input.next().unwrap().unwrap() - 1;
    let mut p2_pos = input.next().unwrap().unwrap() - 1;

    let mut p1_points = 0;
    let mut p2_points = 0;
    let mut dice = 1;

    let win_con = 1000;
    while p1_points < win_con && p2_points < win_con {
        if dice % 2 != 0 {
            p1_pos += 3 * dice + 3;
            p1_points += (p1_pos % 10) + 1;
        } else {
            p2_pos += 3 * dice + 3;
            p2_points += (p2_pos % 10) + 1;
        }
        dice += 3;
    }
    println!("res: {}", [p1_points, p2_points].iter().min().unwrap() * (dice - 1));
}
