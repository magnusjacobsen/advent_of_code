#![feature(stdin_forwarders)]
fn main() {
    println!("{}", (0..256).fold(std::io::stdin().lines().next().unwrap().unwrap().split(',').map(|x| x.parse::<usize>().unwrap() ).fold(vec![0;9], |initial_fish, x| (0..9).map(|i| if i == x { initial_fish[i] + 1 } else { initial_fish[i] }).collect()), |fish, _| (0..9).map(|i| if i == 6 { fish[i + 1] + fish[0] } else if i == 8 { fish[0] }  else { fish[i + 1] }).collect()).iter().sum::<u128>());
}