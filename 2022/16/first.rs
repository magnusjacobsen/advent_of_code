use std::io::BufRead;
use std::collections::{HashMap};

fn get_index(id_str: &str, idxs: &mut HashMap<String, usize>) -> usize {
    if idxs.contains_key(id_str) {
        idxs.get(id_str).unwrap().clone()
    } else {
        let len = idxs.len();
        idxs.insert(id_str.into(), len);
        len
    }
}

fn rec(minutes: i32, open: Vec<i32>,  current: usize, flows: &Vec<i32>, valves: &Vec<Vec<usize>>, mem: &mut HashMap<(usize, i32, Vec<i32>), i32>) -> i32 {
    let pressure = open.iter().sum::<i32>();
    if minutes == 30 {
        return pressure;
    } 
    
    if let Some(val) = mem.get(&(current, minutes, open.clone())) {
        return *val;
    }

    let move_on = valves[current].iter().map(|edge| rec(minutes + 1, open.clone(), *edge, flows, valves, mem)).max().unwrap() + pressure;
    
    if open[current] > 0 || flows[current] == 0 {
        mem.insert((current, minutes, open.clone()), move_on);
        return move_on;
    }
    
    let mut open2 = open.clone();
    open2[current] = flows[current]; 
    let open_valve = rec(minutes + 1, open2, current, flows, valves, mem) + pressure;

    move_on.max(open_valve)
}

fn main() {
    let mut idxs = HashMap::new();
    let mut flows = vec![0; 50];
    let mut valves = vec![vec![]; 50];
    let mut names = vec!["".into(); 50];
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
        names[id] = splt[1].to_string();
    });

    let start = idxs.get("AA").unwrap();

    let mut mem = HashMap::new();
    println!("{}", rec(1, vec![0; idxs.len()], *start, &flows, &valves, &mut mem));
}