use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {

    let mut f = BufReader::new(File::open("input.txt").expect("open failed"));
    let mut marker_value: i32 = 0;

    let mut prev_char: char = ' ';

    for line in f.lines() {
        for c in line.expect("lines failed").chars() {
            if c != prev_char {
                println!("Different char! {}", marker_value)
            }
            prev_char = c;

            marker_value = marker_value + 1;

        }
    }
    println!("Marker after {} characters.", marker_value);
}
