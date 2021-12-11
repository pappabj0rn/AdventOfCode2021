
use std::fs;

fn main() {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-1b\\src\\input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.split("\r\n");

    let mut deeper = 0;
    let mut vector:Vec<i32> = Vec::with_capacity(5);
    
    for line in lines {
        let current =  line.parse::<i32>().unwrap();
        vector.push(current);

        if vector.len() == 4 {
            if compSlices( &vector[0..3], &vector[1..4]) {
                deeper+=1;
            }

            vector.remove(0);  
        }
    }

    println!("\r\ndeeper {}", deeper);
}

fn compSlices(s1: &[i32], s2: &[i32]) -> bool {
    s2.iter().sum::<i32>() > s1.iter().sum::<i32>()
}