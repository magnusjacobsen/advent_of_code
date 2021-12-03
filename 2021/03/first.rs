use std::io::{self, BufRead};

fn main() {
    let bits: Vec<u64> = io::stdin().lock().lines().map(|x| u64::from_str_radix(&x.unwrap(), 2).unwrap()).collect();
    /* 
        first fold is for 0..max bit length
        inside we build a vector of 2 values: gamma and epsilon

        for each bit position we have another fold
        this fold either adds or subtracs 1 from an accumulator, whether a numbers bit in that position is set

        for gamme and epsilon, 
        if the amount of bits for that position is more than half of all numbers (positive number for the accumulator) gamme gets that bit set, otherwise epsilon has that bit set

        finally we fold the gamme-epsilon vector and multiply the numbers
    */
    println!("{}", (0..64 - bits.iter().max().unwrap().leading_zeros() as usize).fold(vec![0_u64;2], |ga_ep, i| if (0..bits.len()).fold(0, |acc,j| if (bits[j] & (1 << i)) != 0 { acc + 1} else { acc - 1} ) > 0 { vec![ga_ep[0] | (1 << i), ga_ep[1]] } else { vec![ga_ep[0], ga_ep[1] | (1 << i)]}).iter().fold(1, |acc, x| acc * x));
}
