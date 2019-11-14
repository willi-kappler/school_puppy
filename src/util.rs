use rustyline::Editor;

use crate::sp_error::SPError;

pub fn input_string() -> Result<Option<String>, SPError> {
    let mut rl = Editor::<()>::new();

    rl.readline("-> ").map(|line| if line == "x" { None } else { Some(line) }).map_err(|_| SPError::ReadLine)
}

pub fn input_number() -> Result<Option<i32>, SPError> {
    match input_string()? {
        Some(line) => line.parse::<i32>().map(|n| Some(n)).map_err(|_| SPError::ParseInt),
        _ => Ok(None)
    }
}
