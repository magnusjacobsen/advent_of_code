use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    println!("{}", 
        // fold over alle dage der skal forløbe
        (0..256).fold(
            // vores startværdi i den ydre fold er baseret på vores input
            io::stdin()
                .lock()
                .lines()
                .next().unwrap().unwrap() // vi bruger next() fordi der kun er én linje med data
                .split(',') // splittes ved komma
                .map(|x| usize::from_str(x).unwrap() ) // konverter string til usize
                .fold( // ny fold der laver en liste af counts af alle med samme værdi
                    // fra input listen, alle med samme værdi grupperes under samme index
                    vec![0; 9], // start værdi
                    |acc, x|  // acc er sum-listen, x er en værdi fra input
                        (0..9).map(|i|
                            if x == i { acc[i] + 1 }
                            else { acc[i] }
                        )
                        .collect()
                ), // slut for opbyggelsen af vores startværdi i den ydre fold
            |acc, _| 
            // i den ydre fold har vi kun brug for en acc, så vores (0..256) værdi giver vi ikke navn
            // denne acc er en liste over hvor mange fisk (værdierne) der deler dagsværdi (index)
            // hver dag laver vi en ny liste baseret på den gamle
            // udfra de få regler der er:
            // * ny index = gammel index - 1
            // * med mindre gammel index er 0, så lægges de til både index 8 og 6
                (0..9).map(|i| 
                    if i == 6 { acc[i + 1] + acc[0] } 
                    else if i == 8 { acc[0] } 
                    else { acc[i + 1] }
                )
                .collect()
        ) // den ydre fold er slut, vi har status ved dag 256
        .iter()
        .sum::<u128>() // og vi summerer
    );
}
