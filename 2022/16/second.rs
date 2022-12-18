use std::io::BufRead;
use std::collections::{HashMap, VecDeque};

fn get_index(id_str: &str, idxs: &mut HashMap<String, usize>) -> usize {
    if idxs.contains_key(id_str) {
        idxs.get(id_str).unwrap().clone()
    } else {
        let len = idxs.len();
        idxs.insert(id_str.into(), len);
        len
    }
}

fn main() {
    let mut idxs = HashMap::new();
    let len = 50;
    let mut flows = vec![0; len];
    let mut valves = vec![vec![]; len];
    std::io::stdin().lock().lines().map(|x| x.unwrap()).for_each(|line| {
        let splt = line.split(" ").collect::<Vec<_>>();
        let id = get_index(splt[1], &mut idxs);
        let flow = splt[4].split("=").nth(1).unwrap().trim_end_matches(&[';']).parse::<i32>().unwrap();
        let edges = splt[9..].iter().map(|e| {
            let e_str = e.trim_end_matches(&[',']);
            get_index(e_str, &mut idxs)
        }).collect::<Vec<_>>();
        flows[id] = flow;
        valves[id] = edges;
    });

    let start = idxs.get("AA").unwrap();

    let mut working = vec![*start];
    (0..len).filter(|i| flows[*i] > 0).for_each(|i| working.push(i));

    let new_flows = (0..working.len()).map(|i| flows[working[i]]).collect::<Vec<_>>();
    let new_valves = (0..working.len()).map(|i| (0..working.len()).filter(|j| i != *j).map(|j| (j as u16, distance(working[i], working[j], &valves) + 1)).collect()).collect::<Vec<Vec<_>>>();

    let mut mem = HashMap::new();
    let result = rec2(0, 0, 0, 0, 1, 0, &new_valves, &new_flows, &mut mem);
    println!("{}", result);
}

fn distance(start: usize, target: usize, valves: &Vec<Vec<usize>>) -> i32 {
    let mut unvisited = vec![true; valves.len()];
    let mut stack = VecDeque::from([(start, 0)]);
    while let Some((current, dist)) = stack.pop_front() {
        if current == target {
            return dist;
        }
        if unvisited[current] {
            unvisited[current] = false;
            for edge in &valves[current] {
                stack.push_back((*edge, dist + 1));
            }
        }
    }
    0
}

#[inline(always)]
fn set(number: u16, index: u16) -> u16 {
    number | (1 << index)
}

#[inline(always)]
fn is_set(number: u16, index: u16) -> i32 {
    ((number >> index) & 1) as i32
}

#[inline(always)]
fn pressure(visited: u16, flows: &Vec<i32>) -> i32 {
    (0..16).map(|i| if is_set(visited, i as u16) == 1 {flows[i]} else {0}).sum::<i32>()
}

fn rec2(ci: u16, cm: i32, ei: u16, em: i32, minutes: i32, visited: u16, valves: &Vec<Vec<(u16, i32)>>, flows: &Vec<i32>, mem: &mut HashMap<(u16, i32, u16, i32, i32, u16), i32>) -> i32 {
    if let Some(val) = mem.get(&(ci, cm, ei, em, minutes, visited)) {
        return *val;
    } else if let Some(val) = mem.get(&(ei, em, ci, cm, minutes, visited)) {
        return *val;
    } else if minutes == 26 {
        let mut new_visited = visited;
        if cm == 0 {
            new_visited = set(new_visited, ci);
        }
        if em == 0 {
            new_visited = set(new_visited, ei);
        }
        return pressure(new_visited, flows);
    }

    let mins_left = 26 - minutes;

    if cm > 0 && em > 0 { // both in travel
        let min = cm.min(em).min(mins_left);
        let val = rec2(ci, cm - min, ei, em - min, minutes + min, visited, valves, flows, mem) + pressure(visited, flows) * min;
        mem.insert((ci, cm, ei, em, minutes, visited), val);
        return val;
    } else if cm > 0 { // elephant moves on
        let new_visited = set(visited, ei);
        if new_visited.count_ones() == flows.len() as u32 {
            return pressure(new_visited, flows) * (mins_left + 1);
        }
        let val = valves[ei as usize].iter().filter(|(edge,_)| is_set(new_visited, *edge) == 0).map(|(edge, dist)| {
            let min = cm.min(*dist).min(mins_left);
            rec2(ci, cm - min, *edge, *dist - min, minutes + min, new_visited, valves, flows, mem) + pressure(new_visited, flows) * min
        }).max().unwrap();

        mem.insert((ci, cm, ei, em, minutes, visited), val);
        return val;
    } else if em > 0 { // we move on
        let new_visited = set(visited, ci);
        if new_visited.count_ones() == flows.len() as u32 {
            return pressure(new_visited, flows) * (mins_left + 1);
        }
        let val = valves[ci as usize].iter().filter(|(edge,_)| is_set(new_visited, *edge) == 0).map(|(edge, dist)| {
            let min = em.min(*dist).min(mins_left);
            rec2(*edge, *dist - min, ei, em - min, minutes + min, new_visited, valves, flows, mem) + pressure(new_visited, flows) * min
        }).max().unwrap();

        mem.insert((ci, cm, ei, em, minutes, visited), val);
        return val;
    } else { //  both move on
        let new_visited = set(set(visited, ei), ci);
        if new_visited.count_ones() == flows.len() as u32 {
            return pressure(new_visited, flows) * (mins_left + 1);
        }
        let val = valves[ci as usize].iter().filter(|(edge,_)| is_set(new_visited, *edge) == 0)
            .map(|(edge, dist)| 
                valves[ei as usize].iter().filter(|(eedge,_)| is_set(new_visited, *eedge) == 0)
                    .map(|(eedge, edist)| {
                        let min = *dist.min(edist).min(&mins_left);
                        rec2(*edge, *dist - min, *eedge, *edist - min, minutes + min, new_visited, valves, flows, mem) + pressure(new_visited, flows) * min
                    }).max().unwrap()
            ).max().unwrap();

        mem.insert((ci, cm, ei, em, minutes, visited), val);
        return val;
    }
}