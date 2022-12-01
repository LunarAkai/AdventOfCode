//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//          Advent of Code - December 1 - Puzzle 1 + 2
//          LunarAkai <https://www.lunarakai.de/>
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut current_cal_value: i32 = 0;
    let mut highest_cal_value: i32 = 0;
    let mut second_highest_cal_value: i32 = 0;
    let mut third_highest_cal_value: i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value) = line {

                match value.parse::<i32>() {
                    Ok(_n) => {
                        let current_line_cal_value: i32 = value.parse().unwrap(); 
                        current_cal_value = current_cal_value + current_line_cal_value;

                        println!("{}", current_cal_value);
                    }
                    Err(_e) => {
                        if value.is_empty() {

                            println!("--- Next Elf ---");

                            if current_cal_value > highest_cal_value  {
                                highest_cal_value = current_cal_value;
                            }
                            if current_cal_value > second_highest_cal_value && current_cal_value < highest_cal_value && current_cal_value > third_highest_cal_value {
                                second_highest_cal_value = current_cal_value;
                            }
                            if current_cal_value > third_highest_cal_value && current_cal_value < second_highest_cal_value {
                                third_highest_cal_value = current_cal_value;
                            }

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

    let top_three_value = highest_cal_value + second_highest_cal_value + third_highest_cal_value;

    println!("Highest Value: {}", highest_cal_value);
    println!("Second Highest Value: {}", second_highest_cal_value);
    println!("Third Highest Value: {}", third_highest_cal_value);
    println!("The three Elves carrying the most calories, carry in total {} calories", top_three_value);
}

// from: rust-by-example
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}