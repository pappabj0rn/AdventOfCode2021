use std::fs;
use std::error::Error;
use std::cmp;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_veritcal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
}

fn main() -> Result<(),Box<dyn Error>> {
    let contents = fs::read_to_string("C:\\GitHub\\AdventOfCode2021\\aoc21-5\\src\\input.txt")?;
    let input_rows:Vec<&str> = contents.split("\r\n").collect();
    let mut x_min=i32::MAX;
    let mut x_max=i32::MIN;
    let mut y_min=i32::MAX;
    let mut y_max=i32::MIN;
    let size:usize = 1000;
    let mut board = vec![vec![0; size]; size];

    for row in input_rows {
        let s = row.replace(" -> ",",");
        let data = 
            s.split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        
        let line = Line{
            start: Point{x: data[0], y: data[1]},
            end: Point{x: data[2], y: data[3]}
        };

        let line_x_max = cmp::max(line.start.x, line.end.x);
        let line_x_min = cmp::min(line.start.x, line.end.x);
        let line_y_min = cmp::min(line.start.y, line.end.y);
        let line_y_max = cmp::max(line.start.y, line.end.y);
        
        x_max = cmp::max(line_x_max, x_max);
        x_min = cmp::min(line_x_min, x_min);
        y_max = cmp::max(line_y_max, y_max);
        y_min = cmp::min(line_y_min, y_min);

        draw_line(&mut board, line);
    }

    println!("x min:{}, x max:{}",x_min, x_max);
    println!("y min:{}, y max:{}",y_min, y_max);    

    //println!("{:?}", board);

    let mut busy_areas = 0;
    for x in 0..size {
        for y in 0..size {
            if board[x as usize][y as usize] > 1 {
                busy_areas += 1;
            }
        }   
    }

    println!("busy:{}",busy_areas);    


    Ok(())
}

fn draw_line(board: &mut Vec<Vec<i32>>, line: Line) {
    if line.is_horizontal(){
        draw_horizontal_line(board, line);
    }
    else if line.is_veritcal() {
        draw_vertical_line(board, line);
    }
    else {
        draw_diagonal_line(board, line);
    }
}

fn draw_vertical_line(board: &mut Vec<Vec<i32>>, line: Line) {
    let y_min = cmp::min(line.start.y, line.end.y);
    let y_max = cmp::max(line.start.y, line.end.y);

    for y in y_min..y_max+1 {
        board[y as usize][line.start.x as usize] += 1;
    }
}

fn draw_horizontal_line(board: &mut Vec<Vec<i32>>, line: Line) {
    let x_min = cmp::min(line.start.x, line.end.x);
    let x_max = cmp::max(line.start.x, line.end.x);

    for x in x_min..x_max+1 {
        board[line.start.y as usize][x as usize] += 1;
    }
}

fn draw_diagonal_line(board: &mut Vec<Vec<i32>>, line: Line) {
    let length = cmp::max(line.start.x, line.end.x) - cmp::min(line.start.x, line.end.x);
    
    let dY = if line.start.y > line.end.y {-1} else {1};
    let dX = if line.start.x > line.end.x {-1} else {1};

    for i in 0..length+1 {
        board[(line.start.y+i*dY)  as usize][(line.start.x+i*dX) as usize] += 1;
    }
}
