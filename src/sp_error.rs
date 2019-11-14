use std::{fmt, fmt::Display, fmt::Formatter};

#[derive(Debug)]
pub enum SPError {
    ReadLine,
    ParseInt,
    Crossterm,
}

// rustyline::error::ReadlineError
// std::num::ParseIntError
// crossterm::utils::error::ErrorKind

impl Display for SPError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            SPError::ReadLine => {
                write!(formatter, "ReadLine error")
            }
            SPError::ParseInt => {
                write!(formatter, "ParseInt error")
            }
            SPError::Crossterm => {
                write!(formatter, "Crossterm error")
            }
        }
    }
}
