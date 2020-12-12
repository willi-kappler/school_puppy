use std::io::{stdout};

use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}};
use log::{info, error, debug};
use rand::{thread_rng, Rng, rngs::ThreadRng, seq::SliceRandom};
use chrono::{NaiveTime, Duration};

use crate::sp_error::SPError;
use crate::util::{input_number, get_number, get_text};

type Questions = Vec<Box<dyn Fn(&mut ThreadRng) -> (u32, bool)>>;

pub fn menu(current_score: u32) -> Result<u32, SPError> {
    let mut score: u32 = current_score;

    loop {
        stdout().execute(Clear(ClearType::All)).map_err(|_| SPError::Crossterm)?;

        println!("Mathemenü"); // Fluent
        println!("Punktestand: {}", score); // Fluent
        println!("Welchen Bereich möchtest du üben ?"); // Fluent
        println!();
        println!("1 - +, -");
        println!("2 - ×, :");
        println!("3 - <, >, =");
        println!("4 - Uhrzeit"); // Fluent
        println!("5 - Längen"); // Fluent
        println!("6 - Gewicht"); // Fluent
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
                    4 => {
                        debug!("Menu 1.4");
                        score += menu1_4();
                    }
                    5 => {
                        debug!("Menu 1.5");
                        score += menu1_5();
                    }
                    6 => {
                        debug!("Menu 1.6");
                        score += menu1_6();
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
            let num1 = rng.gen_range(1, 100001);
            let num2 = rng.gen_range(1, 100001);

            let result = num1 + num2;
            let text = format!("Was ist {} + {} ?", num1, num2); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let mut num1 = rng.gen_range(1, 100001);
            let mut num2 = rng.gen_range(1, 100001);

            if num2 > num1 {
                std::mem::swap(&mut num1, &mut num2);
            }

            let result = num1 - num2;
            let text = format!("Was ist {} - {} ?", num1, num2); // Fluent

            get_number(result, &text)
        })
    ];

    menu_template(questions)
}

fn menu1_2() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(2, 12);
            let num2 = rng.gen_range(2, 12);
            let result = num1 * num2;
            let text = format!("Was ist {} × {} ?", num1, num2); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(2, 12);
            let num2 = rng.gen_range(2, 12);
            let result = num1 * num2;
            let text = format!("Was ist {} : {} ?", result, num1); // Fluent

            get_number(num2, &text)
        }),
        /*
        Box::new(|rng| {
            let num1 = rng.gen_range(10, 1001) * 100;
            let num2 = rng.gen_range(2, 11);
            let result = num1 * num2;
            let text = format!("Was ist {} : {} ?", result, num2); // Fluent

            get_number(num1, &text)
        }),
        */
    ];

    menu_template(questions)
}

fn menu1_3() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 10001);
            let num2 = rng.gen_range(1, 10001);
            let text = format!("{} _ {}", num1, num2);

            if num1 > num2 {
                get_text(">", &text)
            } else if num1 < num2 {
                get_text("<", &text)
            } else {
                get_text("=", &text)
            }
        }),
    ];

    menu_template(questions)
}

fn menu1_4() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let result = num1 * 60;
            let text = format!("Wie viele Minuten sind {} Stunden ?", num1); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let result = num1 * 60;
            let text = format!("Wie viele Stunden sind {} Minuten ?", result); // Fluent

            get_number(num1, &text)
        }),
        Box::new(|rng| {
            let hour = rng.gen_range(0, 24);
            let minute1 = rng.gen_range(0, 60);
            let minute2 = rng.gen_range(1, 120);
            let time1 = NaiveTime::from_hms(hour, minute1, 0);
            let offset = Duration::minutes(minute2);
            let time2 = time1 + offset;
            let text = format!("Wie viele Minuten liegen zwischen {} und {} ?", time1.format("%H:%M"), time2.format("%H:%M")); // Fluent

            get_number(minute2 as i32, &text)
        }),
        Box::new(|rng| {
            let hour1 = rng.gen_range(0, 24);
            let hour2 = rng.gen_range(1, 20);
            let minute = rng.gen_range(0, 60);
            let time1 = NaiveTime::from_hms(hour1, minute, 0);
            let offset = Duration::hours(hour2);
            let time2 = time1 + offset;
            let text = format!("Wie viele Stunden liegen zwischen {} und {} ?", time1.format("%H:%M"), time2.format("%H:%M")); // Fluent

            get_number(hour2 as i32, &text)
        }),
    ];

    menu_template(questions)
}

fn menu1_5() -> u32 {
    let questions: Questions = vec![
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let result = num1 * 100;
            let text = format!("Wieviel cm sind {} m?", num1); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let result = num1;
            let text = format!("Wieviel m sind {} cm?", num1 * 100); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 101);
            let result = num1 * 10;
            let text = format!("Wieviel mm sind {} cm?", num1); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 101);
            let result = num1;
            let text = format!("Wieviel cm sind {} mm?", num1 * 10); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 11);
            let num2 = rng.gen_range(1, 10) * 100;
            let num3 = 1000 - num2;
            let num4 = rng.gen_range(1, 11);
            let result = num1 + 1 + num4;
            let text = format!("Wie lange ist die Reise in km: {} km + {} m + {} km + {} m ?", num1, num2, num4, num3); // Fluent

            get_number(result, &text)
        }),
        Box::new(|rng| {
            let num1 = rng.gen_range(1, 10) * 100;
            let num2 = 1000 - num1;
            let num3 = rng.gen_range(1, 10) * 100;
            let num4 = 1000 - num3;
            let num5 = rng.gen_range(1, 11);

            let result = 2 + num5;
            let text = format!("Wie lange ist die Reise in km: {} m + {} m + {} km + {} m + {} m ?", num4, num2, num5, num3, num1); // Fluent

            get_number(result, &text)
        }),
    ];

    menu_template(questions)
}

fn menu1_6() -> u32 {
    todo!();
    0
}
