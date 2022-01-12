use std::fs;
use std::error::Error;
use std::ops::Add;

struct Board {
    squares: Vec<Square>,
    done: bool,
}

#[derive(Debug)]
struct Square {
    pub value: i32,
    pub marked: bool,
}

impl Board {
    fn new(values: &Vec<i32>) -> Board {
        let s = values
            .into_iter()
            .map(|x| Square{value: x.to_owned(), marked: false})
            .collect::<Vec<_>>();
        
        Board{squares: s, done: false}
    }

    fn print_board(&self) {
        println!("{:?}", self.squares);
    }

    fn mark(&mut self, n:i32){
        for s in &mut self.squares {
            if s.value == n {
                s.mark();
            }
        }
    }

    fn has_bingo(&self) -> bool {
        self.has_marked_row() || self.has_marked_col()
    }

    fn has_marked_row(&self) -> bool {
        self.squares[0..5].iter().all(|x| x.marked)
        || self.squares[5..10].iter().all(|x| x.marked)
        || self.squares[10..15].iter().all(|x| x.marked)
        || self.squares[15..20].iter().all(|x| x.marked)
        || self.squares[20..25].iter().all(|x| x.marked)
    }

    fn has_marked_col(&self) -> bool {
        for i in 0..5 {
            if self.squares[i].marked
                && self.squares[i+5].marked
                && self.squares[i+10].marked
                && self.squares[i+15].marked
                && self.squares[i+20].marked {
                    return true;
                }
        }

        false
    }


    fn print_score(&mut self, drawn_number: i32) {
        let mut unmakred_sum = 0;
        for s in &self.squares {
            if !s.marked {
                unmakred_sum += s.value;
            }
        }

        self.done = true;

        println!("Unmarked sum: {}", unmakred_sum);
        println!("Score: {}", unmakred_sum * drawn_number);
    }
}

impl Square {
    fn mark(&mut self) {
        self.marked = true;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-4\\src\\input.txt")?;
    let lines:Vec<&str> = contents.split("\r\n").collect();

    let mut first = true;

    let mut drawn_numbers = Vec::new();
    let mut values = Vec::new();
    let mut boards = Vec::new();
    for line in lines.into_iter(){
        if first {
            let numbs = line.split(',')
                .map(|x| x.parse::<i32>().unwrap());

            for n in numbs {
                drawn_numbers.push(n);
            }

            first = false;
            continue;
        }

        if line == "\r\n" {
            continue;
        }

        let numbs = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        for n in numbs {
            values.push(n);
        }
        if values.len() == 25 {
            boards.push(Board::new(&values));
            values.clear();
        }
    }

    for n in drawn_numbers {
        for b in &mut boards {
            b.mark(n);
            if b.has_bingo() && !b.done {
                b.print_score(n);
                
            }
        }
    }

    
    Ok(())
}
