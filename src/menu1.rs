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

fn menu_template<F>(min1: i32, max1: i32, min2: i32, max2: i32, questions: Vec<F>) -> u32
    where F: Fn(i32, i32) -> (u32, bool) {

    let mut score: u32 = 0;
    let mut rng = thread_rng();
    let mut num1: i32;
    let mut num2: i32;
    let num_of_operations = questions.len();

    loop {
        num1 = rng.gen_range(min1, max1);
        num2 = rng.gen_range(min2, max2);

        let operation: usize = rng.gen_range(0, num_of_operations);
        let (points, quit) = questions[operation](num1, num2);

        score += points;

        if quit { break }
    }

    score
}

fn menu1_1() -> u32 {
    let questions: Vec<Box<dyn Fn(i32, i32) -> (u32, bool)>> = vec![
        Box::new(|num1, num2| {
            let result = num1 + num2;
            let text = format!("Was ist {} + {} ?", num1, num2); // Fluent
            math_exercise(result, &text)
        }),
        Box::new(|mut num1, mut num2| {
            if num2 > num1 {
                std::mem::swap(&mut num1, &mut num2);
            }
            let result = num1 - num2;
            let text = format!("Was ist {} - {} ?", num1, num2); // Fluent
            math_exercise(result, &text)
        })
    ];

    menu_template(1, 51, 1, 51, questions)
}

fn menu1_2() -> u32 {
    let questions: Vec<Box<dyn Fn(i32, i32) -> (u32, bool)>> = vec![
        Box::new(|num1, num2| {
            let result = num1 * num2;
            let text = format!("Was ist {} × {} ?", num1, num2); // Fluent
            math_exercise(result, &text)
        }),
        Box::new(|num1, num2| {
            let result = num1 * num2;
            let text = format!("Was ist {} : {} ?", result, num1); // Fluent
            math_exercise(num2, &text)
        })
    ];

    menu_template(1, 11, 1, 11, questions)
}

fn math_exercise(result: i32, text: &str) -> (u32, bool) {
    let mut score = 0;

    loop {
        println!("{}", text);

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
