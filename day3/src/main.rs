//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//          Advent of Code - December 3 - Puzzle 1
//          LunarAkai <https://www.lunarakai.de/>
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

use std::collections::HashMap;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let item_type_priorities:HashMap<char, i32> = HashMap::from([
        ('a', 1),('b', 2),('c', 3),('d', 4),('e', 5),('f', 6),('g', 7),('h', 8),('i', 9),('j', 10),('k', 11),('l', 12),('m', 13),('n', 14),
        ('o', 15),('p', 16),('q', 17),('r', 18),('s', 19),('t', 20),('u', 21),('v', 22),('w', 23),('x', 24),('y', 25),('z', 26),
        ('A', 27),('B', 28),('C', 29),('D', 30),('E', 31),('F', 32),('G', 33),('H', 34),('I', 35),('J', 36),('K', 37),('L', 38),('M', 39),('N', 40),('O', 41),('P', 42),('Q', 43),('R', 44),('S', 45),('T', 46),('U', 47),('V', 48),('W', 49),('X', 50),('Y', 51),('Z', 52),
    ]);

    let mut total_priority_value: i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(rucksack_items) = line {

                let rucksack_items_count = rucksack_items.chars().count();

                let rucksack_items_half_count = rucksack_items_count / 2;
                let (first_rs_compartment, second_rs_compartment) = rucksack_items.split_at(rucksack_items_half_count);

                let match_letter = return_matching_letters(first_rs_compartment, second_rs_compartment);

                // match letter with corresponding priority
                if item_type_priorities.contains_key(&match_letter) {
                    let value = item_type_priorities.get(&match_letter);
                    
                    total_priority_value = total_priority_value + value.unwrap();
                }
            }
        }
    }
    println!("Total priority value: {}", total_priority_value);
}

// loop over every letter of first compartment
//      Loop over every letter of second compartment
//          check if letters match 
// return matching letter
fn return_matching_letters(first_string: &str, second_string: &str) -> char {

    let mut matching_letter:char = ' ';

    for letter_first_string in first_string.chars() {
        for letter_second_string in second_string.chars() {
            if letter_first_string == letter_second_string {
                if letter_first_string != matching_letter {
                    matching_letter = letter_first_string;
                }
            }
        }
    }

    return matching_letter;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}