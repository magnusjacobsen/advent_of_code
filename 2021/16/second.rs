#![feature(stdin_forwarders)]
fn to_bits(c: char) -> Vec<u8> {
    match c {
        '0' => vec![0,0,0,0],
        '1' => vec![0,0,0,1],
        '2' => vec![0,0,1,0],
        '3' => vec![0,0,1,1],
        '4' => vec![0,1,0,0],
        '5' => vec![0,1,0,1],
        '6' => vec![0,1,1,0],
        '7' => vec![0,1,1,1],
        '8' => vec![1,0,0,0],
        '9' => vec![1,0,0,1],
        'A' => vec![1,0,1,0],
        'B' => vec![1,0,1,1],
        'C' => vec![1,1,0,0],
        'D' => vec![1,1,0,1],
        'E' => vec![1,1,1,0],
        'F' => vec![1,1,1,1],
        _   => vec![0,0,0,0],
    }
}

fn bits_to_val(slice: &[u8]) -> u32 {
    let mut output = 0;
    for i in 0..slice.len() {
        output |=  (slice[i] as u32) << (slice.len() - i - 1);
    }
    //println!("out: {}, in: {:?}", output, slice);
    output
}

fn concat_bytes(vec: Vec<u32>) -> u64 {
    let mut output = 0;
    for i in 0..vec.len() {
        output |=  (vec[i] as u64) << ((vec.len() - i - 1) * 4);
    }
    output
}

struct Packet {
    value: Option<u64>,
    children: Option<Vec<Packet>>,
    version: u32,
    type_id: u32,
}

impl Packet {
    fn literal(value: u64, version: u32, type_id: u32) -> Self {
        Self {value: Some(value), children: None, version, type_id}
    }

    fn operator(children: Vec<Packet>, version: u32, type_id: u32) -> Self {
        Self {value: None, children: Some(children), version, type_id}
    }

    fn version_sum(&self) -> u32 {
        if let Some(children) = &self.children {
            children.iter().fold(self.version, |acc, c| acc + c.version_sum())
        } else {
            self.version
        }
    }

    fn calculate(&self) -> u64 {
        if let Some(children) = &self.children {
            match self.type_id {
                0 => { // sum
                    children.iter().fold(0_u64, |acc ,x| acc + x.calculate())
                },
                1 => { // product
                    children.iter().fold(1_u64, |acc, x| acc * x.calculate())
                },
                2 => { // minimum
                    children.iter().map(|x| x.calculate()).min().unwrap()
                },
                3 => { // maximum
                    children.iter().map(|x| x.calculate()).max().unwrap()
                },
                5 => { //greater than, always 2 subs
                    if children[0].calculate() > children[1].calculate() { 1 } else { 0 }
                },
                6 => { // less than
                    if children[0].calculate() < children[1].calculate() { 1 } else { 0 }
                },
                7 => { // equal
                    if children[0].calculate() == children[1].calculate() { 1 } else { 0 }
                },
                _ => 0,
            }
        } else {
            self.value.unwrap()
        } 
    }
}

fn parse_packet(i: usize, vec: &Vec<u8>) -> (Packet, usize) {
    let version = bits_to_val(&vec[i..i + 3]);
    let type_id = bits_to_val(&vec[i + 3..i + 6]);
    if type_id == 4 {
        let (literal, new_i) = parse_literal(i + 6, vec);
        (Packet::literal(literal, version, type_id), new_i)
    } else {
        let (children, new_i) = parse_operator(i + 6, vec);
        (Packet::operator(children, version, type_id), new_i)
    }
}

fn parse_operator(mut i: usize, vec: &Vec<u8>) -> (Vec<Packet>, usize) {
    let len_id = if vec[i] == 0 { 15 } else { 11 };
    i += 1;
    let num = bits_to_val(&vec[i..i + len_id]) as usize;
    i += len_id;
    let mut children = vec![];
    if len_id == 15 {
        let stop = i + num;
        while i < stop {
            let (p, new_index) = parse_packet(i, vec);
            children.push(p);
            i = new_index;
        }
    } else {
        for _ in 0..num {
            let (p, new_index) = parse_packet(i, vec);
            children.push(p);
            i = new_index;
        }
    }
    (children, i)
}

fn parse_literal(mut i: usize, vec: &Vec<u8>) -> (u64, usize) {
    let mut literal = vec![];
    loop {
        if vec[i] == 1 {
            literal.push(bits_to_val(&vec[i + 1..i + 5]));
            i += 5;
        } else {
            literal.push(bits_to_val(&vec[i + 1..i + 5]));
            i += 5;
            break;
        }
    }
    let out = concat_bytes(literal);
    (out, i)
}

fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap().chars().fold(vec![], |mut acc, c| {
        acc.extend(to_bits(c));
        acc
    });
    let (p,_) = parse_packet(0, &input);
    println!("{}", p.calculate());
}