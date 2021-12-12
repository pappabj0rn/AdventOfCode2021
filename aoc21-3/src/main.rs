use std::fs;

struct BitCounter {
    one : i32,
    zero : i32,
}

impl BitCounter {
    fn most_common(&self) -> i32 {
        if self.one > self.zero{
            1
        }
        else {
            0
        }
    }
    
    fn least_common(&self) -> i32 {
        if self.one > self.zero{
            0
        }
        else {
            1
        }
    }
}

fn main() {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-3\\src\\input.txt")
        .expect("Something went wrong reading the file");
    let lines = contents.split("\r\n");
    
    let mut bit_counters = vec![
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
        BitCounter { one: 0, zero: 0 },
    ];
    
    let mut l = 1;
    for line in lines {
        let mut i = 0;
        for c in line.chars() {
            match c {
                '0' => bit_counters[i].zero += 1, 
                '1' => bit_counters[i].one += 1,
                _ => continue,
            }            
            i+=1;
        }        
        l+=1;
    }
    
    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;
    
    for i in 0..12 {
        let exp = 11-i;
        let positional_value = base.pow(exp);
        gamma += positional_value * bit_counters[i as usize].MostCommon();
        epsilon += positional_value * bit_counters[i as usize].LeastCommon();
    }
    
    print!("Γ: {}\r\n", gamma);
    print!("Ε: {}\r\n", epsilon);
    print!("mul: {}\r\n", gamma*epsilon);
}
