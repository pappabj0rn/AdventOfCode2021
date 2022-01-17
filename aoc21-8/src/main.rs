use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("src/input.txt")?;
    let rows:Vec<&str> = contents
        .split('\n')
        .collect();

        println!("{:?}",rows.len());
        println!("{:#?}",rows);

    let mut sum = 0;
    for row in rows {
        let mut do_count = false;
        for w in row.split_whitespace() {
            if !do_count && w.len() == 1 {
                do_count = true;
                continue;
            }

            if !do_count {
                continue;
            }

            match w.len() {
                2 | 3 | 4 | 7 => sum+=1,
                _ => continue,                
            }
        }
    }

    println!("{:?}",sum);
        
    Ok(())
}