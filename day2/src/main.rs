//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
//          Advent of Code - December 2 - Puzzle 1 + 2
//          LunarAkai <https://www.lunarakai.de/>
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

// A => Rock
// B => Paper
// C => Scissors

// X => Lose
// Y => Draw
// Z => Win

// Score: 
//  Shape
//      Rock:       1
//      Paper:      2
//      Scissors:   3
//  Outcome: 
//      Loss:       0
//      Draw:       3
//      Win:        6

// Rules: 
//      Rock        >   Scissors
//      Scissors    >   Paper
//      Paper       >   Rock
//      ~           =   ~   =>  Draw

use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum Player_State {
    Rock,
    Paper,
    Scissors,
    NotChosen,
}

fn main() {

    let mut elf_state = Player_State::NotChosen;
    let mut player_state = Player_State::NotChosen;

    let mut player_score: i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(game) = line {
                
                let game_char: Vec<char> = game.chars().collect(); 

                for c in game_char {
                    if c == 'A' {
                        // Elf => Rock
                        elf_state = Player_State::Rock;
                    }
                    if c == 'B' {
                        // Elf => Paper 
                        elf_state = Player_State::Paper;

                    }
                    if c == 'C' {
                        // Elf => Scissors
                        elf_state = Player_State::Scissors;
                        
                    }
                    if c == 'X' {
                        // You => Lose
                        if elf_state == Player_State::Paper {
                            player_state = Player_State::Rock;
                            player_score = player_score + 1;
                        }
                        if elf_state == Player_State::Rock {
                            player_state = Player_State::Scissors;
                            player_score = player_score + 3;
                        }
                        if elf_state == Player_State::Scissors {
                            player_state = Player_State::Paper;
                            player_score = player_score + 2;
                        }
                    }
                    if c == 'Y' {
                        // You => Draw
                        if elf_state == Player_State::Paper {
                            player_state = Player_State::Paper;
                            player_score = player_score + 2;
                        }
                        if elf_state == Player_State::Rock {
                            player_state = Player_State::Rock;
                            player_score = player_score + 1;
                        }
                        if elf_state == Player_State::Scissors {
                            player_state = Player_State::Scissors;
                            player_score = player_score + 3;
                        }
                    }
                    if c == 'Z' {
                        // You => Win
                        if elf_state == Player_State::Paper {
                            player_state = Player_State::Scissors;
                            player_score = player_score + 3;
                        }
                        if elf_state == Player_State::Rock {
                            player_state = Player_State::Paper;
                            player_score = player_score + 2;
                        }
                        if elf_state == Player_State::Scissors {
                            player_state = Player_State::Rock;
                            player_score = player_score + 1;
                        }
                    }
                }

                let mut player_elf_choice = (&elf_state, &player_state);

                match player_elf_choice {
                    //      Elf             Player
                    (Player_State::Rock, Player_State::Rock) => player_score = player_score + 3,            // Draw
                    (Player_State::Rock, Player_State::Paper) => player_score = player_score + 6,           // Win 
                    (Player_State::Rock, Player_State::Scissors) => player_score = player_score + 0,        // Lose

                    (Player_State::Paper, Player_State::Paper) => player_score = player_score + 3,          // Draw
                    (Player_State::Paper, Player_State::Scissors) => player_score = player_score + 6,       // Win
                    (Player_State::Paper, Player_State::Rock) => player_score = player_score + 0,           // Lose

                    (Player_State::Scissors, Player_State::Scissors) => player_score = player_score + 3,    // Draw
                    (Player_State::Scissors, Player_State::Rock) => player_score = player_score + 6,        // Win
                    (Player_State::Scissors, Player_State::Paper) => player_score = player_score + 0,       // Lose

                    (_, _) => panic!("Invalid Input")
                }
            }
        }
    }
    println!("Player score: {}", player_score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}
