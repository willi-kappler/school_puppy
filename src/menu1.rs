use std::io::{stdout};

use crossterm::{ExecutableCommand};
use crossterm::terminal::{Clear, ClearType};
use log::{info, error, debug};
use rand::{thread_rng, Rng};

use crate::sp_error::SPError;
use crate::util::{input_number};

pub fn menu(current_score: u32) -> Result<u32, SPError> {
    let mut score: u32 = current_score;

    loop {
        stdout().execute(Clear(ClearType::All)).map_err(|_| SPError::Crossterm)?;

        println!("Mathemenü"); // Fluent
        println!("Punktestand: {}", score); // Fluent
        println!("Welchen Bereich möchtest du üben ?"); // Fluent
        println!();
        println!("1 - Plus und Minus"); // Fluent
        println!("2 - Mal und Geteilt"); // Fluent
        println!("3 - Uhrzeit"); // Fluent
        println!();
        println!("Bitte gebe eine Zahl ein oder 'x' zum Beenden:"); // Fluent

        match input_number() {
            Ok(None) => {
                debug!("Exit from main menu");
                break
            }
            Ok(Some(number)) => {
                match number {
                    1 => {
                        debug!("Menu 1.1");
                        score += menu1_1();
                    }
                    2 => {
                        debug!("Menu 1.2");
                        score += menu1_2();
                    }
                    3 => {
                        debug!("Menu 1.3");
                    }
                    _ => {
                        error!("Invalid number in menu1: {}", number);
                    }
                }
            }
            Err(error) => {
                error!("Error in menu1: {:?}", error);
            }
        }
    }

    Ok(score)
}

fn menu1_1() -> u32 {
    let mut score: u32 = 0;
    let mut rng = thread_rng();
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    loop {
        num1 = rng.gen_range(1, 50);
        num2 = rng.gen_range(1, 50);

        let operation: u8 = rng.gen_range(0, 2);

        match operation {
            0 => {
                let result = num1 + num2;
                let text = format!("Was ist {} + {} ?", num1, num2); // Fluent
                let (points, quit) = math_exercise(result, &text);

                score += points;

                if quit { break }
            }
            1 => {
                if num2 > num1 {
                    std::mem::swap(&mut num1, &mut num2);
                }
                let result = num1 - num2;
                let text = format!("Was ist {} - {} ?", num1, num2); // Fluent
                let (points, quit) = math_exercise(result, &text);

                score += points;

                if quit { break }
            }
            _ => {
                error!("Unknown operation in menu 1.1: {}", operation)
            }
        }
    }

    score
}

fn menu1_2() -> u32 {
    let mut score: u32 = 0;
    let mut rng = thread_rng();
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;

    loop {
        num1 = rng.gen_range(1, 11);
        num2 = rng.gen_range(1, 11);

        let operation: u8 = rng.gen_range(0, 2);

        match operation {
            0 => {
                let result = num1 * num2;
                let text = format!("Was ist {} × {} ?", num1, num2); // Fluent
                let (points, quit) = math_exercise(result, &text);

                score += points;

                if quit { break }
            }
            1 => {
                let result = num1 * num2;
                let text = format!("Was ist {} : {} ?", result, num1); // Fluent
                let (points, quit) = math_exercise(num2, &text);

                score += points;

                if quit { break }
            }
            _ => {
                error!("Unknown operation in menu 1.2: {}", operation)
            }
        }
    }

    score
}

fn math_exercise(result: i32, text: &str) -> (u32, bool) {
    let mut score = 0;

    loop {
        println!("{}", text); // Fluent

        match input_number() {
            Ok(None) => {
                debug!("Exit from menu 1.1");
                return (score, true)
            }
            Ok(Some(number)) => {
                if number == result {
                    println!("Sehr gut, richtig!"); // Fluent
                    score += 1;
                    return (score, false)
                } else {
                    println!("Das stimmt leider nicht, versuch es bitte noch einmal."); // Fluent
                }
            }
            Err(error) => {
                error!("Error in menu 1.1: {:?}", error);
            }
        }
    }
}
