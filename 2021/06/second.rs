#![feature(stdin_forwarders)]
fn main() {
    println!("{}", (0..256).fold(std::io::stdin().lines().next().unwrap().unwrap().split(',').map(|x| x.parse::<usize>().unwrap() ).fold(vec![0;9], |acc, x| (0..9).map(|i| if i == x { acc[i] + 1 } else { acc[i] }).collect()), |acc, _| (0..9).map(|i| if i == 6 { acc[i + 1] + acc[0] } else if i == 8 { acc[0] }  else { acc[i + 1] }).collect() ).iter().sum::<u128>());
}
