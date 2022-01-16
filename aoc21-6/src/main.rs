use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/input.txt")?;
    let fish:Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut f0:u64 = 0;
    let mut f1 = 0;
    let mut f2 = 0;
    let mut f3 = 0;
    let mut f4 = 0;
    let mut f5 = 0;
    let mut f6 = 0;
    let mut f7 = 0;
    let mut f8 = 0;

    for f in &fish {
        match f {
            0 => f0 += 1,
            1 => f1 += 1,
            2 => f2 += 1,
            3 => f3 += 1,
            4 => f4 += 1,
            5 => f5 += 1,
            6 => f6 += 1,
            7 => f7 += 1,
            8 => f8 += 1,
            _ => panic!("ftw?"),
        }
    }
        
    println!("gen: 0");

    for n in 0..256{
        let spawners = f0;

        f0 = f1;
        f1 = f2;
        f2 = f3;
        f3 = f4;
        f4 = f5;
        f5 = f6;
        f6 = f7+spawners;
        f7 = f8;
        f8 = spawners;
        println!("gen: {}",n+1);
    }

    println!("fish: {}", f0+f1+f2+f3+f4+f5+f6+f7+f8);
    
    Ok(())
}
