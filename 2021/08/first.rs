#![feature(stdin_forwarders)]
fn main() {
   println!("{}", std::io::stdin()
      .lines()
      .map(|x| 
         x.unwrap()
            .split(" | ")
            .collect::<Vec<_>>()[1]
            .split(" ")
            .map(|z| 
               if z.len() == 2 || z.len() == 4 || z.len() == 3 || z.len() == 7 { 
                  1 
               } else { 0 }
            )
            .sum::<u32>()
      )
      .sum::<u32>()
   );
}