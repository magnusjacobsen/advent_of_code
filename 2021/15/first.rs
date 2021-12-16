#![feature(stdin_forwarders)]
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

type Vert = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Vert,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_adjacent(i: usize, j: usize, width: usize) -> Vec<Vert> {
    let mut v = vec![];
    if i > 0 {
        v.push((i - 1, j));
    }
    if i < width - 1 {
        v.push((i + 1, j));
    }
    if j > 0 {
        v.push((i, j - 1));
    }
    if j < width - 1 {
        v.push((i, j + 1));
    }
    v
}

fn main() {
    // strategy: play with dijkstra
    // special thanks to https://doc.rust-lang.org/std/collections/binary_heap/index.html
    let risk = std::io::stdin().lines().map(|x| x.unwrap().chars().map(|y| y.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();

    let width = risk.len();
    let mut dist: HashMap<Vert, usize> = HashMap::new();
    let mut prev: HashMap<Vert, Option<Vert>> = HashMap::new();
    (0..width).for_each(|i| (0..width).for_each(|j| {
        dist.insert((i, j), usize::MAX.clone());
        prev.insert((i, j), None);
    }));
    *dist.get_mut(&(0,0)).unwrap() = risk[0][0];
    let mut heap = BinaryHeap::new();
    heap.push(State { cost: risk[0][0], position: (0,0)});

    let mut final_cost = 0;
    while let Some(State {cost, position}) = heap.pop() {
        if position == (width - 1, width - 1) {
            final_cost = cost;
            break;
        }

        if cost > dist[&position] {
            continue;
        }

        for node in get_adjacent(position.0, position.1, width) {
            let next = State { cost: cost + risk[node.0][node.1], position: node};

            if next.cost < dist[&next.position] {
                heap.push(next);
                *dist.get_mut(&next.position).unwrap() = next.cost;
            }
        }
    }

    println!("{}", final_cost - risk[0][0]);
}