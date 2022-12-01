//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//          Advent of Code - December 1 - Puzzle 1 + 2
//          LunarAkai <https://www.lunarakai.de/>
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_cal_value: i32 = 0;
 
    let mut input:Vec<i32> = vec![];

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {

                match value.parse::<i32>() {
                    Ok(_n) => {
                        let current_line_cal_value: i32 = value.parse().unwrap(); 
                        current_cal_value = current_cal_value + current_line_cal_value;
                    }
                    Err(_e) => {
                        if value.is_empty() {
                            input.push(current_cal_value);

                            current_cal_value = 0;
                            continue;
                        } else {
                            panic!("Couldn't parse value to int32 or line isn't empty");
                        }
                        
                    }   
                }      
            }
        }
    }

    input.sort_by(|b, a| a.partial_cmp(b).unwrap());
    input.truncate(3);

    println!("Highest individual cal value: {:?}", first(&input).unwrap());

    let mut top_three_value = 0;

    for i in input {
        let total: i32 = i;
        top_three_value += total;
    }
    

    println!("The three Elves carrying the most calories, carry in total {} calories", top_three_value);
}

fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

// from: rust-by-example
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}