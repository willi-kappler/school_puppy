use std::io::{stdout, Write};

use crossterm::{execute, ExecutableCommand};
use crossterm::terminal::{Clear, ClearType};
use log::{info, error, debug};
use rustyline::Editor;


pub struct MainMenu {
    score: u32,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu{score: 0}
    }

    pub fn run(self) -> u32 {
        let mut rl = Editor::<()>::new();

        loop {
            match stdout().execute(Clear(ClearType::All)) {
                Ok(_) => {},
                Err(error) => {
                    error!("Error in main menu clear screen:  {:?}", error);
                }
            }

            println!("Hauptmenü"); // Fluent
            println!("Punktestand: {}", self.score); // Fluent
            println!("Welches Fach möchtest du üben ?"); // Fluent
            println!();
            println!("1 - Mathe"); // Fluent
            println!("2 - Deutsch"); // Fluent
            println!("3 - Englisch"); // Fluent
            println!();
            println!("Bitte gebe eine Zahl ein oder 'x' zum Beenden:"); // Fluent
    
            let line = rl.readline("-> ");

            match line {
                Ok(line) => {
                    if line == "x" {
                        debug!("Exit from main menu");
                        break
                    } else {
                        match line.parse::<u32>() {
                            Ok(n) => {
                                match n {
                                    1 => {
                                        // self.score += Menu1::new().run();
                                    }
                                    2 => {
                                        // self.score += Menu2::new().run();
                                    }
                                    3 => {
                                        // self.score += Menu3::new().run();
                                    }
                                    _ => {
                                        error!("Invalid choice in main menu: {}", n);
                                    }
                                }
                            },
                            Err(error) => {
                                error!("Not a number in main menu: {}, {:?}", line, error);
                            }
                        }
                    }
                },
                Err(error) => {
                    error!("An error ocurred in main menu readline(): {:?}", error);
                    break
                }
            }
        }

        self.score
    }

}