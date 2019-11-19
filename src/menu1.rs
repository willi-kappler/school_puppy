use std::io::{stdout};

use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}};
use log::{info, error, debug};
use rand::{thread_rng, Rng, rngs::ThreadRng};
use chrono::{NaiveTime, Duration};

use crate::sp_error::SPError;
use crate::util::{input_number};

type Questions = Vec<Box<dyn Fn(&mut ThreadRng) -> (u32, bool)>>;

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
                        score += menu1_3();
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

fn menu_template<F>(questions: Vec<F>) -> u32
    where F: Fn(&mut ThreadRng) -> (u32, bool) {

    let mut score: u32 = 0;
    let mut rng = thread_rng();
    let num_of_operations = questions.len();

    loop {
        let operation: usize = rng.gen_range(0, num_of_operations);
        let (points, quit) = questions[operation](&mut rng);

        score += points;

        if quit { break }
    }

    score
}

fn menu1_1() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 51);
            let num2 = rng.gen_range(1, 51);
            let result = num1 + num2;
            let text = format!("Was ist {} + {} ?", num1, num2); // Fluent
            math_exercise(result, &text)
        }),
        Box::new(|rng| {
            let mut num1 = rng.gen_range(1, 51);
            let mut num2 = rng.gen_range(1, 51);
            if num2 > num1 {
                std::mem::swap(&mut num1, &mut num2);
            }
            let result = num1 - num2;
            let text = format!("Was ist {} - {} ?", num1, num2); // Fluent
            math_exercise(result, &text)
        })
    ];

    menu_template(questions)
}

fn menu1_2() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let num2 = rng.gen_range(1, 11);
            let result = num1 * num2;
            let text = format!("Was ist {} × {} ?", num1, num2); // Fluent
            math_exercise(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let num2 = rng.gen_range(1, 11);
            let result = num1 * num2;
            let text = format!("Was ist {} : {} ?", result, num1); // Fluent
            math_exercise(num2, &text)
        }),
    ];

    menu_template(questions)
}

fn menu1_3() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let result = num1 * 60;
            let text = format!("Wie viele Minuten sind {} Stunden ?", num1); // Fluent
            math_exercise(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let result = num1 * 60;
            let text = format!("Wie viele Stunden sind {} Minuten ?", result); // Fluent
            math_exercise(num1, &text)
        }),
        Box::new(|rng| {
            let hour = rng.gen_range(0, 24);
            let minute1 = rng.gen_range(0, 60);
            let minute2 = rng.gen_range(1, 120);
            let time1 = NaiveTime::from_hms(hour, minute1, 0);
            let offset = Duration::minutes(minute2);
            let time2 = time1 + offset;


            let text = format!("Wie viele Minuten liegen zwischen {} und {} ?", time1.format("%H:%M:%S"), time2.format("%H:%M:%S")); // Fluent
            math_exercise(minute2 as i32, &text)
        }),
        Box::new(|rng| {
            let hour1 = rng.gen_range(0, 24);
            let hour2 = rng.gen_range(1, 20);
            let minute = rng.gen_range(0, 60);
            let time1 = NaiveTime::from_hms(hour1, minute, 0);
            let offset = Duration::hours(hour2);
            let time2 = time1 + offset;


            let text = format!("Wie viele Stunden liegen zwischen {} und {} ?", time1.format("%H:%M:%S"), time2.format("%H:%M:%S")); // Fluent
            math_exercise(hour2 as i32, &text)
        }),
    ];

    menu_template(questions)
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
