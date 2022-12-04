#![feature(stdin_forwarders)]
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

const COSTS: [u32; 4] = [1, 10, 100, 1000];
const GOAL: [[usize; 2]; 4] = [[7, 8], [9, 10], [11, 12], [13, 14]];

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    heuristic: u32,
    state: [u8; 15],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn abs(a: u32, b: u32) -> u32 {
    if a > b { a - b } else { b - a }
}

fn distance(position: (u32, u32), target: (u32, u32)) -> u32 {
    if position.1 == target.1 { 
        abs(position.0, target.0) 
    } else {
        abs(position.1, target.1) + position.0 + target.0 
    }
}

fn heuristic_cost(state: &[u8; 15], distance_map: &[[u32; 15]; 15]) -> u32 {
    let mut cost = 0;
    for i in 0..state.len() {
        let cur = state[i] as usize;
        if cur == 255 || i == GOAL[cur][1] || (i == GOAL[cur][0] && state[GOAL[cur][0]] == cur as u8) {
            //println!("i: {} = empty", i);
            continue;
        }
        let this = distance_map[i][GOAL[cur][0]] * COSTS[cur];
        cost += this;
        //println!("i: {}, this: {}, total: {}", i, this, cost);
    }

    for i in 0..4 {
        if state[GOAL[i][1]] != i as u8 {
            cost += COSTS[i];
            //println!("i: {}, this: {}, total: {}", i,  COSTS[i], cost);
        } 
    }

    cost
}

fn distance_map() -> [[u32; 15]; 15] {
    let pos = vec![(0, 0), (0, 1), (0, 3), (0, 5), (0, 7), (0, 9), (0, 10), (1, 2), (2, 2), (1, 4), (2, 4), (1, 6), (2, 6), (1, 8), (2, 8)];
    let mut map = [[0; 15]; 15];
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            map[i][j] = distance(pos[i], pos[j]);
        }
    }
    map
}

fn is_finished(state: &[u8;15]) -> bool {
    state[7] == 0 && state[8] == 0 && state[9] == 1 && state[10] == 1 && state[11] == 2 && state[12] == 2 && state[13] == 3 && state[14] == 3
}

fn is_allowed(from: usize, to: usize, state: &[u8; 15]) -> bool {
    state[to] == 255 && match to {
        7 => from == 8 || state[from] == 0,
        8 => state[from] == 0,
        9 => from == 10 || state[from] == 1,
        10 => state[from] == 1,
        11 => from == 12 || state[from] == 2,
        12 => state[from] == 2,
        13 => from == 14 || state[from] == 3,
        14 => state[from] == 3,
        _ => true,
    }
}

fn get_moves(state: &[u8; 15], graph: &Vec<Vec<usize>>, distance_map: &[[u32; 15]; 15]) -> Vec<([u8; 15], u32)> {
    let mut vec = vec![];
    for from in 0..state.len() {
        let cur = state[from];
        if cur < 255 {
            for to in &graph[from] {
                if is_allowed(from, *to, state) {
                    let mut clone = state.clone();
                    clone[from] = 255;
                    clone[*to] = cur;
                    vec.push((clone, distance_map[*to][from] * COSTS[cur as usize]));
                }
            }
        }
    }
    vec
}

fn main() {
    let input = std::io::stdin().lines().map(|x| x.unwrap().chars().map(|y| y as u32).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut state = [255; 15];
    state[7] = (input[2][3] - 'A' as u32) as u8;
    state[8] = (input[3][3] - 'A' as u32) as u8;
    state[9] = (input[2][5] - 'A' as u32) as u8;
    state[10] = (input[3][5] - 'A' as u32) as u8;
    state[11] = (input[2][7] - 'A' as u32) as u8;
    state[12] = (input[3][7] - 'A' as u32) as u8;
    state[13] = (input[2][9] - 'A' as u32) as u8;
    state[14] = (input[3][9] - 'A' as u32) as u8;

    let graph = vec![
        vec![1],            // 0
        vec![0, 2, 7],      // 1
        vec![1, 3, 7, 9],   // 2
        vec![2, 4, 9, 11],  // 3
        vec![3, 5, 11, 13], // 4
        vec![4, 6, 13],     // 5
        vec![5],           // 6
        vec![1, 2, 8],      // 7  A
        vec![7],            // 8  A
        vec![2, 3, 10],     // 9  B
        vec![9],            // 10 B
        vec![3, 4, 12],     // 11 C
        vec![11],           // 12 C
        vec![4, 5, 14],     // 13 D
        vec![13],           // 14 D
    ];

    let distances = distance_map();
    
    
    let start_h = heuristic_cost(&state, &distances);
    println!("start h: {}", start_h);
    
    
    let mut heap = BinaryHeap::new();
    for (next, cost) in get_moves(&state, &graph, &distances) {
        let heuristic = cost + heuristic_cost(&next, &distances);
        heap.push(State {cost: cost, heuristic: heuristic, state: next});
    }
    let mut visited: HashMap<[u8;15], u32> = HashMap::new();

    while let Some(s) = heap.pop() {
        if is_finished(&s.state) {
            println!("\n{:?}", s.state);
            println!("h: {}", s.heuristic);
            println!("cost: {}", s.cost);
            break;
        }
        if let Some(former_cost) = visited.get(&s.state) {
            if *former_cost <= s.cost {
                continue;
            }
        }
        visited.insert(s.state, s.cost);
        //println!("{}, {}, {:?}", s.cost, s.heuristic, s.state);
        for (next_state, next_cost) in get_moves(&s.state, &graph, &distances) {
            let heuristic = s.cost + next_cost + heuristic_cost(&next_state, &distances);
            heap.push(State {cost: s.cost + next_cost, heuristic: heuristic, state: next_state});
        } 
    }
}

