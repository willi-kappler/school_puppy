use std::io::{stdout};

use crossterm::{ExecutableCommand};
use crossterm::terminal::{Clear, ClearType};
use log::{info, error, debug};

use crate::sp_error::SPError;
use crate::util::{input_number};
use crate::menu1;
use crate::menu3;

pub fn menu() -> Result<u32, SPError> {
    let mut score: u32 = 0;

    loop {
        stdout().execute(Clear(ClearType::All)).map_err(|_| SPError::Crossterm)?;

        println!("Hauptmenü"); // Fluent
        println!("Punktestand: {}", score); // Fluent
        println!("Welches Fach möchtest du üben ?"); // Fluent
        println!();
        println!("1 - Mathe"); // Fluent
        println!("2 - Deutsch"); // Fluent
        println!("3 - Englisch"); // Fluent
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
                        debug!("Menu 1");
                        match menu1::menu(score) {
                            Ok(new_score) => score = new_score,
                            Err(error) => error!("Error in menu1: {:?}", error),
                        }
                    }
                    2 => {
                        debug!("Menu 2");
                    }
                    3 => {
                        debug!("Menu 3");
                        match menu3::menu(score) {
                            Ok(new_score) => score = new_score,
                            Err(error) => error!("Error in menu3: {:?}", error),
                        }
                    }
                    _ => {
                        error!("Invalid number in main menu: {}", number);
                    }
                }
            }
            Err(error) => {
                error!("Error in main menu: {:?}", error);
            }
        }
    }

    Ok(score)
}
