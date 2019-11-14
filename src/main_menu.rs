use std::io::{stdout, Write};

use crossterm::{execute, ExecutableCommand};
use crossterm::terminal::{Clear, ClearType};
use log::{info, error, debug};

use crate::sp_error::SPError;
use crate::util::{input_number};

pub fn main_menu() -> Result<u32, SPError> {
    let mut score = 0;

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
                    }
                    2 => {
                        debug!("Menu 2");
                    }
                    3 => {
                        debug!("Menu 3");
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
