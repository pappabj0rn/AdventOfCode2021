use std::fs;
use std::error::Error;

struct SegmentDecoder {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
}
/*

    a
f       b
    g
e       c
    d 
 
 */

impl SegmentDecoder {
    fn new() -> SegmentDecoder{
        SegmentDecoder{
            a:"".to_owned(),
            b:"".to_owned(),
            c:"".to_owned(),
            d:"".to_owned(),
            e:"".to_owned(),
            f:"".to_owned(),
            g:"".to_owned(),
        }
    }

    fn init(&mut self, hints: Vec<&str>){
        let four = self.extract_four(&hints);                
        self.apply_seven(four, &hints);
        self.handle_069(&hints);
    }

    fn extract_four<'a>(&mut self, hints: &Vec<&'a str>) -> &'a str {
        let four = hints
            .iter()
            .enumerate()
            .filter(|&(_, &hint)| hint.len()  == 4)
            .map(|(_, &hint)| hint)
            .collect::<Vec<&str>>()[0];
        self.b = four.clone().to_owned();
        self.c = four.clone().to_owned();
        self.f = four.clone().to_owned();
        self.g = four.clone().to_owned();
        four
    }

    fn apply_seven(&mut self, four: &str, hints: &Vec<&str>) {
        let mut remaining = "abcdefg".to_owned();
        for c in four.chars() {
            remaining = remaining.replace(c,"");
        }
        let seven = hints
            .iter()
            .enumerate()
            .filter(|&(_, &hint)| hint.len()  == 3)
            .map(|(_, &hint)| hint)
            .collect::<Vec<&str>>()[0];
        let seven_a = seven.chars().enumerate()
            .filter(|&(_,c)| remaining.contains(c))
            .map(|(_,c)| c)
            .collect::<Vec<char>>()[0];
        let one = seven.replace(seven_a, "");

        remaining = remaining.replace(seven_a, "");
        self.a = seven_a.to_string();
        self.b = one.clone();
        self.c = one.clone();
        self.d = remaining.clone();
        self.e = remaining.clone();

        for c in one.chars() {
            self.f = self.f.replace(c,"");
        }
        self.g = self.f.clone();
    }

    fn handle_069(&mut self, hints: &Vec<&str>) {
        let six_segment_digits = hints
            .iter()
            .enumerate()
            .filter(|&(_, &hint)| hint.len()  == 6)
            .map(|(_, &hint)| hint)
            .collect::<Vec<&str>>();
        
        
        let (_0, _1) = SegmentDecoder::special_split(&self.b);

        let six = SegmentDecoder::get_digit_matching_c1_or_c2(&six_segment_digits, &_0, &_1);
        
        match six.contains(_0) {
            true => {
                self.b = _1.to_string(); 
                self.c = _0.to_string()
            },
            false => {
                self.b = _0.to_string(); 
                self.c = _1.to_string()
            },
        }

        let (_0, _1) = SegmentDecoder::special_split(&self.f);

        let zero = SegmentDecoder::get_digit_matching_c1_or_c2(&six_segment_digits, &_0, &_1);

        match zero.contains(_0) {
            true => {
                self.f = _0.to_string(); 
                self.g = _1.to_string();
            },
            false => {
                self.f = _1.to_string(); 
                self.g = _0.to_string();
            },
        }

        let (_0, _1) = SegmentDecoder::special_split(&self.d);

        let nine = SegmentDecoder::get_digit_matching_c1_or_c2(&six_segment_digits, &_0, &_1);

        match nine.contains(_0) {
            true => {
                self.d = _0.to_string(); 
                self.e = _1.to_string();
            },
            false => {
                self.d = _1.to_string(); 
                self.e = _0.to_string();
            },
        }
    }

    fn special_split(str:&str) -> (char, char) {
        let mut c1 = ' ';
        let mut c2 = ' ';
        let mut i = 0;
        for c in str.chars() {
            if i == 0 {
                c1 = c;
            }

            if i == 1 {
                c2 = c;
            }    
            i+=1;        
        }

        (c1,c2)
    }

    fn get_digit_matching_c1_or_c2<'a>(six_segment_digits: &Vec<&'a str>, c1:&char, c2:&char) -> &'a str {
        six_segment_digits
            .iter()
            .enumerate()
            .filter(|&(_, &x)| x.contains(*c1) ^ x.contains(*c2))
            .map(|(_, &x)| x)
            .collect::<Vec<&str>>()[0]
    }

    fn decode(&self, input: Vec<&str>) -> i32 {
        let mut sum = 0;
        let mut pos_value = 1000;
        for str in input {
            let d = self.decode_digit(&str);
            sum += d*pos_value;
            pos_value /= 10;
        }

        sum
    }

    fn decode_digit(&self, input: &str) -> i32 {
        if input.len() == 2{
            return 1;
        }
        if input.len() == 3{
            return 7;
        }
        if input.len() == 4{
            return 4;
        }
        if input.len() == 7{
            return 8;
        }

        if input.contains(self.a.chars().nth(0).unwrap())
        && input.contains(self.c.chars().nth(0).unwrap())
        && input.contains(self.d.chars().nth(0).unwrap())
        && input.contains(self.e.chars().nth(0).unwrap())
        && input.contains(self.f.chars().nth(0).unwrap())
        && input.contains(self.g.chars().nth(0).unwrap()) {
            return 6;
        }

        if input.contains(self.a.chars().nth(0).unwrap()) 
        && input.contains(self.b.chars().nth(0).unwrap()) 
        && input.contains(self.d.chars().nth(0).unwrap()) 
        && input.contains(self.e.chars().nth(0).unwrap()) 
        && input.contains(self.g.chars().nth(0).unwrap()) {
            return 2;
        }

        if input.contains(self.a.chars().nth(0).unwrap())
        && input.contains(self.b.chars().nth(0).unwrap())
        && input.contains(self.c.chars().nth(0).unwrap())
        && input.contains(self.d.chars().nth(0).unwrap())
        && input.contains(self.f.chars().nth(0).unwrap())
        && input.contains(self.g.chars().nth(0).unwrap()) {
            return 9;
        }

        if input.contains(self.a.chars().nth(0).unwrap())
        && input.contains(self.c.chars().nth(0).unwrap())
        && input.contains(self.d.chars().nth(0).unwrap())
        && input.contains(self.f.chars().nth(0).unwrap())
        && input.contains(self.g.chars().nth(0).unwrap()) {
            return 5;
        }

        if input.contains(self.a.chars().nth(0).unwrap())
        && input.contains(self.b.chars().nth(0).unwrap())
        && input.contains(self.c.chars().nth(0).unwrap())
        && input.contains(self.d.chars().nth(0).unwrap())
        && input.contains(self.g.chars().nth(0).unwrap()) {
            return 3;
        }

        0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/input.txt")?;
    let lines = contents.lines();

    let mut sum = 0;
    for line in lines {
        let input:Vec<&str> = line
            .split('|')
            .collect();

        println!("{:?}",line);
        let mut decoder = SegmentDecoder::new();
        decoder.init(input[0].split_whitespace().collect());
        let decode = decoder.decode(input[1].split_whitespace().collect());
        println!("{:?}",decode);
        sum += decode;
    }

    assert_ne!(1192424, sum);
    assert_ne!(990417, sum); //to low
    
    println!("{:?}",sum);
        
    Ok(())
}