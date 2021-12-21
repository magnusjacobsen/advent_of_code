#![feature(stdin_forwarders)]
use std::collections::HashMap;

fn roll() -> Vec<u32> {
    (1..4).map(|i| (1..4).map(move |j| (1..4).map(move |k| k + j + i ) ).flatten() ).flatten().collect()
}

fn rec(turn: u32, dice: u32, mut p1: (u32, u32), mut p2: (u32, u32), mem: &mut HashMap<(u32, (u32, u32), (u32, u32)), (u128, u128)>) -> (u128, u128) {
    if turn > 0 {
        if turn % 2 != 0 {
            p1.0 += dice;
            p1.1 += (p1.0 % 10) + 1;
        } else {
            p2.0 += dice;
            p2.1 += (p2.0 % 10) + 1;
        }
    }

    if p1.1 > 20 {
        return (1, 0);
    } else if p2.1 > 20 {
        return (0, 1);
    } else if let Some(v) = mem.get(&(turn, p1, p2)) {
        return *v;
    }

    let mut res = (0, 0);
    for d in roll() {
        let qres = rec(turn + 1, d, p1, p2, mem);
        res = (res.0 + qres.0, res.1 + qres.1);
    }
    mem.insert((turn, p1, p2), res);
    return res;

}

fn main() {
    let mut input = std::io::stdin().lines().map(|x| x.unwrap().split(": ").collect::<Vec<_>>()[1].parse::<u32>());
    let p1_pos = input.next().unwrap().unwrap() - 1;
    let p2_pos = input.next().unwrap().unwrap() - 1;
    let mut map = HashMap::new();
    let res = rec(0, 0, (p1_pos, 0), (p2_pos, 0), &mut map);
    println!("{}", if res.0 > res.1 { res.0 } else { res.1 });
}
