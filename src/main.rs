// External crates
use log::{info, error, debug};
use log4rs;

mod util;
mod sp_error;
mod main_menu;
mod menu1;

// TODO: Use fluent: https://github.com/projectfluent/fluent-rs/tree/master/fluent-resmgr

// TODO: Maybe use anyhow: https://github.com/dtolnay/anyhow

fn main() {

    let file_logger = log4rs::append::file::FileAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{d} {l} - {m}{n}")))
        .build("school_puppy.log").unwrap();

    let log_config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("file_logger", Box::new(file_logger)))
        .build(log4rs::config::Root::builder().appender("file_logger").build(log::LevelFilter::Debug))
        .unwrap();

    let _log_handle = log4rs::init_config(log_config).unwrap();

    info!("Running school puppy..");

    match main_menu::menu() {
        Ok(score) => {
            println!("Deine Punkte: {}", score); // Fluent
        }
        Err(error) => {
            error!("Error in main menu: {:?}", error);
        }
    }
    
    println!("Tschüß bis zum nächsten Mal!"); // Fluent
}
