use std::io::{stdout};

use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}};
use log::{info, error, debug};
use rand::{thread_rng, Rng, rngs::ThreadRng, seq::SliceRandom};
use chrono::{NaiveTime, Duration};

use crate::sp_error::SPError;
use crate::util::{input_number, get_text};

type Questions = Vec<Box<dyn Fn(&mut ThreadRng) -> (u32, bool)>>;

pub fn menu(current_score: u32) -> Result<u32, SPError> {
    let mut score: u32 = current_score;

    loop {
        stdout().execute(Clear(ClearType::All)).map_err(|_| SPError::Crossterm)?;

        println!("Englischmenü"); // Fluent
        println!("Punktestand: {}", score); // Fluent
        println!("Welchen Bereich möchtest du üben ?"); // Fluent
        println!();
        println!("1 - Teil 1");
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
                        debug!("Menu 3.1");
                        score += menu3_1();
                    }
                    _ => {
                        error!("Invalid number in menu3: {}", number);
                    }
                }
            }
            Err(error) => {
                error!("Error in menu3: {:?}", error);
            }
        }
    }

    Ok(score)
}

fn menu3_1() -> u32 {
    0
}