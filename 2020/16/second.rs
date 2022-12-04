use std::io::{self, BufRead};
use std::collections::HashMap;

fn parse_input() -> (Vec<Vec<i32>>, Vec<i32>, Vec<Vec<i32>>, Vec<usize>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut rules: Vec<Vec<i32>> = vec![];
    let mut departures: Vec<usize> = vec![];

    // getting all the rules
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        if tmp == "" { break; }
        let ranges: Vec<i32> = tmp
            .split(": ")
            .collect::<Vec<&str>>()[1]
            .split(" or ")
            .map(|x| x
                .split("-")
                .map(|y| y
                    .parse::<i32>()
                    .unwrap())
                .collect::<Vec<i32>>())
            .flatten()
            .collect();
        rules.push(ranges);
        if tmp.contains("departure") {
            departures.push(rules.len() - 1);
        }
    }

    // getting my own ticket
    lines.next();
    let my_ticket: Vec<i32> = lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    // scanning other people's tickets
    lines.next();
    lines.next();
    let mut tickets: Vec<Vec<i32>> = vec![];
    while let Some(v) = lines.next() {
        let tmp = v.unwrap();
        tickets.push(tmp.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
    }
    (rules, my_ticket, tickets, departures)
}

fn filter_tickets(rules: Vec<Vec<i32>>, tickets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut valid_tickets = vec![];
    'ticket: for i in 0..tickets.len() {
        'field: for j in 0..tickets[0].len() {
            let value = tickets[i][j];
            for k in 0..rules.len() {
                if (value >= rules[k][0] && value <= rules[k][1]) || (value >= rules[k][2] && value <= rules[k][3]) {
                    continue 'field;
                }
            }
            continue 'ticket;
        }
        valid_tickets.push(tickets[i].clone());
    }
    valid_tickets
}

fn valid_rules(rules: Vec<Vec<i32>>, tickets: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut valid_rules: Vec<Vec<usize>> = vec![vec![]; tickets[0].len()]; // ticket field_id => rule_ids
    for k in 0..rules.len() {
        'field: for j in 0..tickets[0].len() {
            for i in 0..tickets.len() {
                let value = tickets[i][j];
                if !((value >= rules[k][0] && value <= rules[k][1]) || (value >= rules[k][2] && value <= rules[k][3])) {
                    continue 'field;
                }
            }
            valid_rules[j].push(k);
        }
    }
    valid_rules
}

// possible rules = ticket field id, rule_ids
fn field_id_to_rule_id(possible_rules: &Vec<Vec<usize>>, field_index: usize, configuration: &mut HashMap<usize, usize>) -> bool {
    'rule: for j in 0..possible_rules[field_index].len() {
        if configuration.contains_key(&j) {
            continue 'rule;
        }
        configuration.insert(j, field_index);
        if field_index == possible_rules.len() - 1 {
            return true;
        }
        let is_possible = field_id_to_rule_id(possible_rules, field_index + 1, configuration);
        if is_possible {
            return is_possible;
        } else {
            configuration.remove(&j);
        }
    }
    false
}

fn main() {
    let (rules, my_ticket, tickets, departure_ids) = parse_input();
    println!("len tickets: {}", tickets.len());
    let valid_tickets = filter_tickets(rules.clone(), tickets);
    println!("len valid tickes: {}", valid_tickets.len());
    let valid_rules = valid_rules(rules, valid_tickets);
    let mut configuration: HashMap<usize, usize> = HashMap::new();
    let is_possible = field_id_to_rule_id(&valid_rules, 0, &mut configuration);
    
    println!("\nDeparture values:");
    let mut mult: u128 = 1;
    for dids in departure_ids {
        let value = my_ticket[configuration[&dids]];
        mult *= value as u128;
        println!("{}", value);
    }
    println!("mult: {}", mult);
}
