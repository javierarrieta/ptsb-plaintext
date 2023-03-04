mod data;
mod ptsb;

use std::env;
use std::string::String;
use crate::data::AppError;
use std::io;

impl From<csv::Error> for AppError {
    fn from(value: csv::Error) -> Self {
        return AppError::InvalidData(value.to_string())
    }
}

fn main() -> Result<(), AppError> {
    let arguments: Vec<String> = env::args().collect();
    let fallback_path = env::var("PTSB_XLS_FILE").ok().unwrap_or("".to_string());
    let f_from_args = arguments.get(1);
    let f = f_from_args
        .unwrap_or(&fallback_path);

    let rows = ptsb::parse_file(f)?;

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for row in rows {
        wtr.serialize(row)?
    }

    wtr.flush()?;

    return Ok(())
}
