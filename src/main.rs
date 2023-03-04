mod data;
mod ptsb;

use std::env;
use std::env::VarError;
use std::string::String;
use crate::data::AppError;
use std::io;

impl From<csv::Error> for AppError {
    fn from(value: csv::Error) -> Self {
        return AppError::InvalidData(value.to_string())
    }
}

impl From<VarError> for AppError {
    fn from(value: VarError) -> Self {
        return AppError::InvalidArgument(value.to_string())
    }
}

fn main() -> Result<(), AppError> {
    let arguments: Vec<String> = env::args().collect();
    let f = arguments.get(1)
        .ok_or(AppError::InvalidArgument("No path provided as argument".to_string()))?;

    let mut rows = ptsb::parse_file(f)?;
    rows.sort_by(|a,b| a.partial_cmp(b).unwrap());

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for row in rows {
        wtr.serialize(row)?
    }

    wtr.flush()?;

    return Ok(())
}
