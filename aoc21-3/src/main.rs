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
    
    let mut l = 1;
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
        l+=1;
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


    let x = format!("{:b}", gamma);
    print!("{:?}", x);
    //let ogr = find_ogr(&lines, 0, gamma);
    //let cosr = find_cosr(&lines, 0, epsilon);

    //let bin_idx = "01110011001";
    //let intval = isize::from_str_radix(bin_idx, 2).unwrap();
    //print!("ogr: {}\r\n", ogr);
    //print!("cosr: {}\r\n", cosr);
    //print!("mul: {}\r\n", ogr*cosr);
}

fn matches_mc(line:&str, i:&usize) -> bool {
    false
}

fn find_ogr(lines:&Vec<&str>, iteration:usize) -> String {
    
    if lines.len() == 1 {
        return String::from(lines[0]);
    }
    
    if lines.len() == 2 {
        if lines[0].ends_with("1") {
            return String::from(lines[0]);
        }
        else {
            return String::from(lines[1]);
        }
    }

    let lines = lines.into_iter()
        .filter(|l| matches_mc(l,&iteration))
        .cloned()
        .collect::<Vec<&str>>();

    return find_ogr(&lines, iteration+1);
}

fn find_cosr<'a>(lines:&'a Vec<&str>, iteration:usize) -> &'a str {
    return lines[0]
}