use std::io::BufRead;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    op: Op,
    div: i32,
    test_true: usize,
    test_false: usize,
}

#[derive(Debug)]
enum Op {
    Mult(i32),
    Add(i32),
    MultSelf,
}

fn inspect_all(monkey: &Monkey) -> Vec<i32> {
    monkey.items.iter().map(|x|
        match monkey.op {
            Op::Mult(num) => x * num,
            Op::Add(num) => x + num,
            Op::MultSelf => x * x,
        } / 3
    ).collect::<Vec<_>>()
}

fn main() {
    let mut lines = std::io::stdin().lock().lines().collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>();
    let mut monkeys = vec![];
    let mut inspections = vec![];
    while let Some(line) = lines.pop() {
        if line.unwrap() == "" { continue; }
        let items_str = lines.pop().unwrap().unwrap();
        let items = items_str.split("Starting items: ").collect::<Vec<_>>()[1].split(", ").map(|y| y.parse().unwrap()).collect::<Vec<i32>>();
        let op_str = lines.pop().unwrap().unwrap();
        let op_str2 = op_str.trim().split(" ").collect::<Vec<_>>();
        let op = match op_str2[4] {
            "*" => {
                match op_str2[5] {
                    "old" => Op::MultSelf,
                    num => Op::Mult(num.parse().unwrap()),
                }
            },
            "+" => Op::Add(op_str2[5].parse().unwrap()),
            _ => panic!("shouldnt be here 2"),
        };
        let div_str = lines.pop().unwrap().unwrap();
        let div = div_str.trim().split(" ").collect::<Vec<_>>()[3].parse().unwrap();
        let test_true_str = lines.pop().unwrap().unwrap();
        let test_true = test_true_str.trim().split(" ").collect::<Vec<_>>()[5].parse().unwrap();
        let test_false_str = lines.pop().unwrap().unwrap();
        let test_false = test_false_str.trim().split(" ").collect::<Vec<_>>()[5].parse().unwrap();
        let monkey = Monkey {items, op, div, test_true, test_false};
        monkeys.push(monkey);
        inspections.push(0);
    }

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            let worries = inspect_all(&monkeys[i]);
            monkeys[i].items = vec![];
            inspections[i]+= worries.len();
            let test_true = monkeys[i].test_true;
            let test_false = monkeys[i].test_false;
            for worry in worries {
                if worry % monkeys[i].div == 0 {
                    monkeys[test_true].items.push(worry);
                } else {
                    monkeys[test_false].items.push(worry);
                }
            }
        }
    }

    inspections.sort();
    inspections.reverse();
    println!("{}", inspections[0] * inspections[1]);
}