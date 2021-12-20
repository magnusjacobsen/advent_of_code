#![feature(stdin_forwarders)]
// always pad images by zeros for 4 positions on all sides
fn pad_image(image: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out = vec![vec![0; image[0].len() + 8]; image.len() + 8];
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            out[i + 4][j + 4] = image[i][j];
        }
    }
    out
}

fn read_image(i: usize, j: usize, image: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = vec![0; 9];
    out[0] = image[i - 1][j - 1];
    out[1] = image[i - 1][j];
    out[2] = image[i - 1][j + 1];
    out[3] = image[i][j - 1];
    out[4] = image[i][j];
    out[5] = image[i][j + 1];
    out[6] = image[i + 1][j - 1];
    out[7] = image[i + 1][j];
    out[8] = image[i + 1][j + 1];
    out
}

fn to_index(vec: Vec<u8>) -> usize {
    (0..vec.len()).fold(0_usize, |out, i| out | (vec[i] as usize) << (vec.len() - i - 1))
}

fn count_ones(image: &Vec<Vec<u8>>) -> usize {
    image.iter().flat_map(|x| x.iter()).filter(|pixel| **pixel == 1).count()
}

fn main() {
    let input = std::io::stdin().lines().map(|x| x.unwrap().chars().map(|y| if y == '.' { 0 } else { 1 }).collect::<Vec<_>>()).collect::<Vec<_>>();
    let enhancement = input[0].clone();

    let mut image = pad_image((2..input.len()).map(|i| input[i].clone()).collect::<Vec<_>>());
    for step in 0..50 {
        let fill = if step % 2 == 0 { enhancement[0] } else if enhancement[0] == 1 { enhancement[511] } else { 0 }; 
        let mut new = vec![vec![fill; image[0].len() + 8]; image.len() + 8];
        for i in 1..image.len() - 1 {
            for j in 1..image.len() - 1 {
                let index = to_index(read_image(i, j, &image));
                new[i + 5][j + 5] = enhancement[index];     
            }
        }
        image = new;
    }
    
    println!("{}", count_ones(&image));
}
