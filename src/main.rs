// use crossterm::{
//     event::{self, Event as CEvent, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
//use promptly::prompt;
//use tui::{backend::CrosstermBackend, Terminal};
use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Player {
    First,
    Opponent,
    Tie,
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum ParseChoiceError {
    //  FailedParseString(String),
    NotValidChoice,
}

impl Choice {
    fn get_winner(first: &Choice, second: &Choice) -> Player {
        // if [first,second].iter().all(|&choice| choice == Choice::Rock || choice == Choice::Scissors)
        // if [first,second].contains(Choice::Rock) {
        //     unimplemented!();
        // }

        match (first, second) {
            (Choice::Rock, Choice::Scissors) => Player::First,
            (Choice::Scissors, Choice::Rock) => Player::Opponent,
            (_, _) => {
                if first < second {
                    Player::Opponent
                } else if first > second {
                    Player::First
                } else {
                    Player::Tie
                }
            }
        }
    }
}

impl FromStr for Choice {
    type Err = ParseChoiceError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.trim().to_lowercase().as_ref() {
            "rock" | "r" => Ok(Choice::Rock),
            "paper" | "p" => Ok(Choice::Paper),
            "scissors" | "s" => Ok(Choice::Scissors),
            _ => Err(ParseChoiceError::NotValidChoice),
        }
    }
}

#[derive(Debug)]
struct Computer {
    turn: Player,
}

impl Default for Computer {
    fn default() -> Computer {
        Computer {
            turn: Player::First,
        }
    }
}

#[derive(Debug)]
struct Friend {
    turn: Player,
}

impl Default for Friend {
    fn default() -> Friend {
        Friend {
            turn: Player::First,
        }
    }
}

#[derive(Debug)]
struct Online {
    turn: Player,
}

impl Default for Online {
    fn default() -> Online {
        Online {
            turn: Player::First,
        }
    }
}

trait GameHandler {
    fn get_second_players_turn() -> Choice;

    fn play(&self) {
        loop {
            match input::<String>()
                .msg("Choose a format! ((B)est of n/(E)ndless/(R)ace to n/(Q)uit): ")
                .get()
                .trim()
                .to_uppercase()
                .as_str()
            {
                "ABORT" | "EXIT" | "Q" | "QUIT" => break,
                // "B" | "BEST" | "BEST OF N" => do_best(),
                // "E" | "ENDLESS" => do_endless(),
                "R" | "RACE" | "RACE TO N" => Self::do_race(),
                _ => continue,
            }
        }
    }

    fn play_a_round() -> Player {
        Choice::get_winner(
            &input::<Choice>()
                .msg("Choose a move! (R/P/S): ")
                .err("Please enter a valid move! (R/P/S): ")
                .get(),
            &Self::get_second_players_turn(),
        )
    }

    // fn do_best() {
    //     unimplemented!()
    // }

    // fn do_endless() {
    //     loop {
    //         play_a_round()
    //     }
    // }

    fn do_race() {
        let goal: i32 = input()
            .msg("Please enter how much the player has to score in order to win: ")
            .err("Please enter a valid number! ")
            .get();
        let mut score1 = 0;
        let mut score2 = 0;
        while score1 < goal && score2 < goal {
            match Self::play_a_round() {
                Player::First => {
                    score1 += 1;
                    println!("You won!");
                }
                Player::Opponent => {
                    score2 += 2;
                    println!("You won!");
                }
                Player::Tie => {
                    println!("It's a tie!");
                }
            };
        }
    }
}

impl GameHandler for Computer {
    fn get_second_players_turn() -> Choice {
        Choice::Rock
    }
}

impl GameHandler for Friend {
    fn get_second_players_turn() -> Choice {
        Choice::Rock
    }
}

impl GameHandler for Online {
    fn get_second_players_turn() -> Choice {
        Choice::Rock
    }
}

fn abort() {
    std::process::exit(0);
}

fn help() {
    println!("(H)elp  => Prints this help screen");
    println!("(Q)uit | Abort | Exit  => Exit the app");
}

fn computer() {
    let handler = Computer::default();
    handler.play();
}

fn friend() {
    let handler = Friend::default();
    handler.play();
}

fn online() {
    let handler = Online::default();
    handler.play();
}

fn main() {
    loop {
        println!("Welcome to Rock Paper Scissors!");
        println!("You can play against a (c)omputer, your (f)riend online, or a strange (o)nline!");

        match input::<String>()
            .msg("Which one do you prefer? (computer/friend/online): ")
            .get()
            .trim()
            .to_uppercase()
            .as_str()
        {
            "ABORT" | "EXIT" | "Q" | "QUIT" => abort(),
            "HELP" | "H" => help(),
            "COMPUTER" | "C" => computer(),
            "FRIEND" | "F" => friend(),
            "ONLINE" | "O" => online(),
            _ => continue,
        }
    }
}
