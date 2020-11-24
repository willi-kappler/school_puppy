use rustyline::Editor;
use log::{info, error, debug};

use crate::sp_error::SPError;

pub fn input_text() -> Result<Option<String>, SPError> {
    let mut rl = Editor::<()>::new();

    rl.readline("-> ").map(|line| if line == "x" { None } else { Some(line) }).map_err(|_| SPError::ReadLine)
}

pub fn input_number() -> Result<Option<i32>, SPError> {
    match input_text()? {
        Some(line) => line.parse::<i32>().map(|n| Some(n)).map_err(|_| SPError::ParseInt),
        _ => Ok(None)
    }
}

pub fn get_number(result: i32, text: &str) -> (u32, bool) {
    let mut score = 0;

    loop {
        println!("{}", text);

        match input_number() {
            Ok(None) => {
                debug!("Exit from menu");
                return (score, true)
            }
            Ok(Some(number)) => {
                if number == result {
                    score += 1;

                    println!("Sehr gut, richtig! Deine Punkte: {}", score);  // Fluent

                    return (score, false)
                } else {
                    println!("Das stimmt leider nicht, versuch es bitte noch einmal."); // Fluent
                }
            }
            Err(error) => {
                error!("Error in menu get_number: {:?}", error);
            }
        }
    }
}

pub fn get_text(result: &str, text: &str) -> (u32, bool) {
    let mut score = 0;

    loop {
        println!("{}", text);

        match input_text() {
            Ok(None) => {
                debug!("Exit from menu");
                return (score, true)
            }
            Ok(Some(text)) => {
                if text == result {
                    println!("Sehr gut, richtig!"); // Fluent
                    score += 1;
                    return (score, false)
                } else {
                    println!("Das stimmt leider nicht, versuch es bitte noch einmal."); // Fluent
                }
            }
            Err(error) => {
                error!("Error in menu get_text: {:?}", error);
            }
        }
    }
}
