#![feature(stdin_forwarders)]
use std::collections::HashMap;

fn main() {
    let mut paths = 0;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let input = std::io::stdin().lines().map(|x| x.unwrap().split("-").map(|y| y.to_string()).collect::<Vec<_>>()).collect::<Vec<_>>();

    // lets build the graph
    for pair in input {
        if !map.contains_key(&pair[0]) {
            map.insert(pair[0].clone(), vec![pair[1].clone()]);
        } else {
            map.get_mut(&pair[0]).unwrap().push(pair[1].clone());
        }
        if !map.contains_key(&pair[1]) {
            map.insert(pair[1].clone(), vec![pair[0].clone()]);
        } else {
            map.get_mut(&pair[1]).unwrap().push(pair[0].clone());
        }
    }
   
    let mut queue = vec![("start", vec!["start"])];
    while queue.len() > 0 {
        let (current, path) = queue.pop().unwrap();
        for node in map.get(current).unwrap() {
            let mut new_path = path.clone();
            new_path.push(node);
            if node == "start" {
                continue;
            }
            if node == "end" {
                paths += 1;
                continue;
            } 
            if node.chars().all(|x| x.is_ascii_lowercase()) && path.iter().any(|y| y == node) {
                continue;
            }
            queue.push((node, new_path));
        }
    }
    println!("paths: {}", paths);
}
