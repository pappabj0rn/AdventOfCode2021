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
        if self.one < self.zero{
            1
        }
        else {
            0
        }
    }

    fn same(&self) -> bool {
        self.most_common() == self.least_common()
    }
}

fn main() {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-3\\src\\input.txt")
        .expect("Something went wrong reading the file");
    let lines:Vec<&str> = contents.split("\r\n").collect();
    
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
    
    for line in &lines {
        let mut i = 0;
        for c in line.chars() {
            match c {
                '0' => bit_counters[i].zero += 1, 
                '1' => bit_counters[i].one += 1,
                _ => continue,
            }            
            i+=1;
        }    
    }
    
    let mut gamma = 0;
    let mut epsilon = 0;
    let base: i32 = 2;
    
    for i in 0..12 {
        let exp = 11-i;
        let positional_value = base.pow(exp);
        gamma += positional_value * bit_counters[i as usize].most_common();
        epsilon += positional_value * bit_counters[i as usize].least_common();
    }
    
    print!("Γ: {}\r\n", gamma);
    print!("Ε: {}\r\n", epsilon);
    print!("mul: {}\r\n", gamma*epsilon);


    let ogr = find_ogr(&lines, 0);    
    let cosr = find_cosr(&lines, 0);
    println!("ogr: {}",ogr);
    println!("cosr: {}",cosr);

    let ogr = bin_str_to_i32(&ogr);
    let cosr = bin_str_to_i32(&cosr);
    println!("ogr: {}",ogr);
    println!("cosr: {}",cosr);
    println!("mul: {}", ogr*cosr);
}

fn bin_str_to_i32(input: &str) -> i32 {
    let base: i32 = 2;
    let mut accumulator = 0;
    let mut i = 0;
    for c in input.chars() {
        let exp = 11-i;
        let positional_value = base.pow(exp);
        let d = if c == '1' {1} else {0};
        accumulator += positional_value * d;
        i+=1;
    }

    accumulator
}

fn matches_mc(line:&str, bc:&BitCounter, i:&usize) -> bool {
    match line.chars().nth(i.to_owned()).unwrap() {
        '1' => bc.most_common() == 1 || bc.same(),
        '0' => bc.most_common() == 0 && !bc.same(),
        _ => false,
    }
}

fn matches_lc(line:&str, bc:&BitCounter, i:&usize) -> bool {
    match line.chars().nth(i.to_owned()).unwrap() {
        '1' => bc.least_common() == 1 && !bc.same(),
        '0' => bc.least_common() == 0 || bc.same(),
        _ => false,
    }
}

fn find_ogr(lines:&Vec<&str>, iteration:usize) -> String {
    
    if lines.len() == 1 {
        return String::from(lines[0]);
    }

    let mut bc = BitCounter { one: 0, zero: 0 };
    for line in lines {
        match line.chars().nth(iteration).unwrap() {
            '0' => bc.zero += 1, 
            '1' => bc.one += 1,
            _ => continue,
        }   
    }

    let lines = lines.into_iter()
        .filter(|l| matches_mc(l, &bc,&iteration))
        .cloned()
        .collect::<Vec<&str>>();

    find_ogr(&lines, iteration+1)
}

fn find_cosr(lines:&Vec<&str>, iteration:usize) -> String {
    
    if lines.len() == 1 {
        return String::from(lines[0]);
    }

    let mut bc = BitCounter { one: 0, zero: 0 };
    for line in lines {
        match line.chars().nth(iteration).unwrap() {
            '0' => bc.zero += 1, 
            '1' => bc.one += 1,
            _ => continue,
        }   
    }

    let lines = lines.into_iter()
        .filter(|l| matches_lc(l, &bc,&iteration))
        .cloned()
        .collect::<Vec<&str>>();

    find_cosr(&lines, iteration+1)
}