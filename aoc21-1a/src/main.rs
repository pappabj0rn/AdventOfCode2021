
use std::fs;

fn main() {

    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-1a\\src\\input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.split("\r\n");

    let mut prev = 1000;
    let mut deeper = 0;
    let mut tot = 0;

    for line in lines {
        let current =  line.parse::<i32>().unwrap();
        if current > prev {
            deeper+=1;            
        }

        prev = current;  
        tot+=1;      
    }
    println!("deeper {}", deeper);
    println!("tot {}", tot);
}