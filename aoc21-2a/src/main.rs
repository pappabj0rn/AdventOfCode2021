use std::fs;

#[derive(Debug)]
enum Direction {
    None,
    Forward,
    Down,
    Up
}

fn main() {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-2a\\src\\input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.split("\r\n");
    
    let mut depth = 0;
    let mut hpos = 0;
    let mut aim = 0;
    
    for line in lines {
        let values = line.split(" ");
        let mut distance:i32;
        let mut direction = Direction::None;
        
        for v in values {
            match direction {
                Direction::None => {
                    direction = match v.chars().next() {
                        Some('f') => Direction::Forward,
                        Some('d') => Direction::Down,
                        Some('u') => Direction::Up,
                        _ => Direction::None,
                    };
                },
                _ => {
                    distance = v.parse::<i32>().unwrap();
                    match direction {
                        Direction::Forward => { 
                            hpos += distance;
                            depth += aim * distance;
                        },
                        Direction::Down => aim += distance,
                        Direction::Up => aim += distance * -1,
                        _ => print!("wat"),
                    }
                }
            }
        }
    }
    print!("hpos {}\r\n",hpos);
    print!("depth {}\r\n",depth);
    print!("mul {}",depth * hpos);
}

