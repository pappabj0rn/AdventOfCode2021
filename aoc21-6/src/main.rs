use std::fs;
use std::error::Error;
use std::cmp;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-6\\src\\input.txt")?;
    let mut fish:Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
        
    for _ in 0..80{        
        for i in 0..fish.len() {
            let mut f = fish[i]-1;

            if f == -1 {
                f = 6;
                fish.push(8);
            }

            fish[i] = f;
        }
    }

    println!("fish: {}", fish.len());
    
    Ok(())
}
