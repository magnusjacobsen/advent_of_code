#![feature(stdin_forwarders)]
// Yay we are gonna play with recursive types again today!

#[derive(Clone)]
struct Pair {
    literal: Option<i32>,
    pair: Vec<Pair>,
}

impl Pair {
    fn literal(value: i32) -> Self {
        Self {literal: Some(value), pair: vec![] }
    }

    fn pair(a: Pair, b: Pair) -> Self {
        Self {literal: None, pair: vec![a, b]}
    }

    fn print(&self) {
        if let Some(v) = &self.literal {
            print!("{}", v);
        } else {
            print!("["); 
            self.pair[0].print();
            print!(",");
            self.pair[1].print();
            print!("]");
        }
    }

    fn is_literal(&self) -> bool {
        self.literal.is_some()
    }

    fn is_pair(&self) -> bool {
        self.pair.len() > 0
    }

    fn explode(&mut self, depth: usize, exploded: bool, mut left: i32, mut right: i32) -> (bool, i32, i32) {
        if self.pair.len() > 0 {
            // we already exploded
            if exploded { 
                if let Some(v) = self.pair[0].literal {
                    self.pair[0] = Pair::literal(v + right);
                    right = 0;
                } else {
                    let (_, new_right, _) = self.pair[0].explode(depth + 1, exploded, 0, right);
                    right = new_right;
                }
                if let Some(v) = self.pair[1].literal {
                    self.pair[1] = Pair::literal(v + left);
                    left = 0;
                } else {
                    let (_,new_left,_) = self.pair[1].explode(depth + 1, exploded, left, 0);
                    left = new_left;
                }
                return (true, left, right);
            } else if depth > 4 && self.pair[0].is_literal() && self.pair[1].is_literal() { // now we explode
                let left = self.pair[0].literal.unwrap();
                let right = self.pair[1].literal.unwrap();
                self.literal = Some(0);
                self.pair = vec![];
                return (true, left, right);
            } else { // we are not yet exploded
                let (exploded, left, right) = self.pair[0].explode(depth + 1, false, 0, 0);
                if exploded { // left side exploded
                    if let Some(v) = self.pair[1].literal {
                        self.pair[1] = Pair::literal(v + right);
                        return (exploded, left, 0);
                    } else {
                        let (_, _, new_right) = self.pair[1].explode(depth + 1, exploded, 0, right);
                        return (true, left, new_right);
                    }
                } else {
                    let (exploded, left, right) = self.pair[1].explode(depth + 1, false, 0, 0);
                    if exploded {
                        if let Some(v) = self.pair[0].literal {
                            self.pair[0] = Pair::literal(v + left);
                            return (true, 0, right);
                        } else {
                            let (_, new_left, _) = self.pair[0].explode(depth + 1, exploded, left, 0);
                            return (true, new_left, right);
                        }
                    }
                }
            }
        }
        (false, 0, 0)
    }

    fn split(&mut self) -> bool {
        if let Some(v) = self.literal {
            if v > 9 {
                self.pair = vec![Pair::literal(v / 2), Pair::literal((v + 1) / 2)];
                self.literal = None;
                return true;
            }
            return false;
        } else {
            self.pair[0].split() || self.pair[1].split()
        }
    }

    fn magnitude(&self) -> i32 {
        if let Some(v) = self.literal {
            v
        } else {
            3 * self.pair[0].magnitude() + 2 * self.pair[1].magnitude()
        }
    }
}

fn parse(i: usize, vec: &Vec<char>) -> (Pair, usize) {
    match vec[i] {
        '[' => {
            let (a, new_i1) = parse(i + 1, vec);
            let (b, new_i2) = parse(new_i1 + 1, vec);
            (Pair::pair(a, b), new_i2 + 1)
        },
        _ => (Pair::literal(vec[i].to_digit(10).unwrap() as i32), i + 1),
    }
}

fn reduce(pair: &mut Pair) {
    'split: loop {
        'explode: loop {
            let (expl, _, _) = pair.explode(1, false, 0, 0);
            if !expl {
                break 'explode;
            }
        }
        if !pair.split() {
            break 'split;
        }
    }
}

// they should already be reduced
fn add(a: Pair, b: Pair) -> Pair {
    let mut new = Pair::pair(a, b);
    reduce(&mut new);
    new
}

fn main() {
    let input = std::io::stdin().lines().map(|x| { let (pair,_) = parse(0, &x.unwrap().chars().collect::<Vec<_>>()); pair }).collect::<Vec<_>>();
    
    let mut best = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let mut a = input[i].clone();
            let mut b = input[j].clone();
            reduce(&mut a);
            reduce(&mut b);
            let res = add(a, b);
            let mag = res.magnitude();
            if best < mag {
                best = mag;
            }
        }
    }

    println!("{}", best);
}
