use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/input.txt")?;
    let crab_input:Vec<i32> = contents
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let max = crab_input.iter().max().unwrap(); 
    
    let mut crabs =vec![0;(*max+1) as usize];
    let mut calc_space =vec![0;(*max+1) as usize];

    for c in crab_input {
        crabs[c as usize] += 1;
    }

    let len = crabs.len();
    for c in 0..len {        
        for i in 0..len {
            if i == c {
                continue;
            }
            let x = i as i32 - c as i32;
            calc_space[i] += (crabs[c]*x).abs();
        }
    }

    println!("{:?}",crabs);
    
    println!("min:{}",calc_space.iter().min().unwrap());

    Ok(())
}